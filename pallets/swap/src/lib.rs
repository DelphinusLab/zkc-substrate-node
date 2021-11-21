#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use delphinus_crypto::{BabyJubjub, BabyJubjubField, BabyJubjubPoint, Curve, PrimeField, EDDSA};
use delphinus_crypto::Encode as BabyJubJubEncode;
use frame_support::traits::{Currency, ReservableCurrency};
use frame_support::{decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use num_bigint::{BigInt, Sign};
use sp_core::{U256};

mod aux;
mod errors;
mod types;

use aux::*;
use errors::*;
use types::*;

#[cfg(test)]
mod mock;

pub trait Config: frame_system::Config {
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type ADMIN1: Get<<Self as frame_system::Config>::AccountId>;
    type ADMIN2: Get<<Self as frame_system::Config>::AccountId>;
}

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const ACK1: u8 = 1u8;
const ACK2: u8 = 2u8;
const NACK: u8 = ACK1 | ACK2;
const PENDING: u8 = 1u8;
const DONE: u8 = 2u8;

const MAX_ACCOUNT_COUNT: u32 = 1u32 << 20;
const MAX_TOKEN_COUNT: u32 = 1u32 << 10;
const MAX_POOL_COUNT: u32 = 1u32 << 10;

const OP_DEPOSIT: u8 = 0u8;
const OP_WITHDRAW: u8 = 1u8;
const OP_SWAP: u8 = 2u8;
const OP_RETRIEVE: u8 = 3u8;
const OP_SUPPLY: u8 = 4u8;
const OP_ADDPOOL: u8 = 5u8;
const OP_SETKEY: u8 = 6u8;

decl_event!(
    pub enum Event<T>
    where
        Balance = BalanceOf<T>,
        BlockNumber = <T as frame_system::Config>::BlockNumber,
    {
        SetKey(ReqId, AccountIndex, PublicKey),
        Deposit(ReqId, Signature, AccountIndex, TokenIndex, Amount),
        WithdrawReq(
            ReqId,
            Signature,
            AccountIndex,
            TokenIndex,
            Amount,
            L1Account,
            NonceId,
        ),
        SwapReq(
            ReqId,
            Signature,
            AccountIndex,
            PoolIndex,
            Amount,
            Direction,
            NonceId,
        ),
        PoolSupplyReq(
            ReqId,
            Signature,
            AccountIndex,
            PoolIndex,
            Amount,
            Amount,
            NonceId,
        ),
        PoolRetrieveReq(
            ReqId,
            Signature,
            AccountIndex,
            PoolIndex,
            Amount,
            Amount,
            NonceId,
        ),
        Ack(ReqId, u8),
        Abort(ReqId),
        RewardFunds(AccountIndex, Balance, BlockNumber),
        AddPoolReq(ReqId, Signature, PoolIndex, TokenIndex, TokenIndex),
    }
);

decl_storage! {
    trait Store for Module<T: Config> as SimpleMap {
        pub AccountIndexCount: AccountIndex;
        pub AccountIndexMap get(fn account_index_map): map hasher(blake2_128_concat) T::AccountId => Option<AccountIndex>;

        pub PoolIndexCount: PoolIndex;
        pub PoolIndexMap get(fn pool_index_map): map hasher(blake2_128_concat) (TokenIndex, TokenIndex) => Option<PoolIndex>;

        pub BalanceMap get(fn balance_map): map hasher(blake2_128_concat) (AccountIndex, TokenIndex) => Amount;
        pub ShareMap get(fn share_map): map hasher(blake2_128_concat) (AccountIndex, PoolIndex) => Amount;
        pub PoolMap get(fn pool_map): map hasher(blake2_128_concat) PoolIndex => (TokenIndex, TokenIndex, Amount, Amount);

        pub PendingReqMap get(fn pending_req_map): map hasher(blake2_128_concat) ReqId => Option<Ops>;
        pub CompleteReqMap get(fn complete_req_map): map hasher(blake2_128_concat) ReqId => Option<Ops>;
        pub AckMap get(fn ack_map): map hasher(blake2_128_concat) ReqId => u8;
        pub ReqIndex get(fn req_index): ReqId;
        pub NonceMap get(fn nonce_map): map hasher(blake2_128_concat) T::AccountId => NonceId;
        pub KeyMap get(fn key_map): map hasher(blake2_128_concat) AccountIndex => Option<PublicKey>;

        pub DepositMap get(fn deposit_map): map hasher(blake2_128_concat) ReqId => Option<L1TxHash>;
        pub L1TxMap get(fn l1txhash_map): map hasher(blake2_128_concat) L1TxHash => u8;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// Awards the specified amount of funds to the specified account
        #[weight = 0]
        pub fn charge(origin, account: T::AccountId, reward: BalanceOf<T>) {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let _r = T::Currency::deposit_creating(&account, reward);
            let now = <frame_system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::RewardFunds(_account_index, reward, now));
            return Ok(());
        }

        #[weight = 0]
        pub fn set_key(origin, key: [u8; 32]) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            let key = BabyJubjubPoint::decode(&key).map_err(|_| Error::<T>::InvalidKey)?;

            let x = u256_from_bigint(&key.x.v);
            let y = u256_from_bigint(&key.y.v);

            let req_id = req_id_get::<T>()?;
            let account_index = create_account_index::<T>(&_who)?;

            KeyMap::insert(account_index, (x, y));
            ReqIndex::put(req_id);

            Self::deposit_event(RawEvent::SetKey(req_id, account_index, (x, y)));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn add_pool(
            origin,
            sign: [u8; 64],
            token_index_0: TokenIndex,
            token_index_1: TokenIndex,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            is_admin::<T>(&who)?;

            if token_index_0 >= MAX_TOKEN_COUNT {
                return Err(Error::<T>::InvalidTokenIndex)?;
            }

            if token_index_1 >= MAX_TOKEN_COUNT {
                return Err(Error::<T>::InvalidTokenIndex)?;
            }

            if token_index_0 == token_index_1 {
                return Err(Error::<T>::InvalidTokenPair)?;
            }

            let who_account_index = get_account_index::<T>(&who)?;
            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_ADDPOOL;
            command[1..9].copy_from_slice(&nonce.to_le_bytes());
            command[9..13].copy_from_slice(&token_index_0.to_le_bytes());
            command[13..17].copy_from_slice(&token_index_1.to_le_bytes());
            // command[17..49] and command[49..81] is reserved in current implementataion.
            let sign = check_sign::<T>(who_account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            let (_token_index_0, _token_index_1) =
                if token_index_0 < token_index_1 {
                    (token_index_0, token_index_1)
                } else {
                    (token_index_1, token_index_0)
                };

            let pool_index = create_pool_index::<T>(&_token_index_0, &_token_index_1)?;
            let op = Ops::AddPool(sign, pool_index, token_index_0, token_index_1);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(Event::<T>::AddPoolReq(req_id, sign, pool_index, token_index_0, token_index_1));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn deposit(
            origin,
            sign: [u8; 64],
            account_index: AccountIndex,
            token_index: TokenIndex,
            amount: Amount,
            l1_tx_hash: L1TxHash,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let who_account_index = get_account_index::<T>(&who)?;

            if token_index >= AccountIndexCount::get() {
                return Err(Error::<T>::InvalidAccount)?;
            }

            if L1TxMap::get(l1_tx_hash) != 0u8 {
                return Err(Error::<T>::L1TXExists)?;
            }

            if amount == U256::from(0) {
                return Err(Error::<T>::InvalidAmount)?;
            }

            let _new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_DEPOSIT;
            command[1..9].copy_from_slice(&nonce.to_le_bytes());
            command[9..13].copy_from_slice(&account_index.to_le_bytes());
            command[13..17].copy_from_slice(&token_index.to_le_bytes());
            command[17..49].copy_from_slice(&amount.to_le_bytes());
            command[49..81].copy_from_slice(&l1_tx_hash.to_le_bytes());
            let sign = check_sign::<T>(who_account_index, &command, &sign)?;

            let _req_id = req_id_get::<T>()?;

            let _new_balance_amount = balance_add::<T>(&account_index, &token_index, amount)?;
            let op = Ops::Deposit(sign, account_index, token_index, amount);

            balance_set(&account_index, &token_index, _new_balance_amount);
            PendingReqMap::insert(&_req_id, op);
            ReqIndex::put(_req_id);
            NonceMap::<T>::insert(&who, _new_nonce);
            DepositMap::insert(&_req_id, l1_tx_hash);
            L1TxMap::insert(&l1_tx_hash, PENDING);

            Self::deposit_event(Event::<T>::Deposit(_req_id, sign, account_index, token_index, amount));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn withdraw(
            origin,
            sign: Signature,
            account: T::AccountId,
            l1account: L1Account,
            token_index: TokenIndex,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;

            let _req_id = req_id_get::<T>()?;
            let _new_nonce = nonce_check::<T>(&account, nonce)?;
            let _new_balance = balance_sub::<T>(&_account_index, &token_index, amount)?;

            let op = Ops::Withdraw(
                sign,
                _account_index, token_index, amount, l1account, nonce);
            PendingReqMap::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &token_index, _new_balance);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(Event::<T>::WithdrawReq(
                _req_id,
                sign,
                _account_index, token_index, amount, l1account, nonce
            ));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn swap(
            origin,
            sign: Signature,
            account: T::AccountId,
            token_from: TokenIndex,
            token_to: TokenIndex,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let _token_from_index = token_from;
            let _token_to_index = token_to;
            let _direction =  if _token_from_index < _token_to_index { 0u8 } else { 1u8 };
            let _pool_index =
                if _direction == 0 {
                    get_pool_index::<T>(&_token_from_index, &_token_to_index)?
                } else {
                    get_pool_index::<T>(&_token_to_index, &_token_from_index)?
                };

            let _req_id = req_id_get::<T>()?;
            let _new_nonce = nonce_check::<T>(&account, nonce)?;

            // for user account
            let _new_balance_from = balance_sub::<T>(&_account_index, &_token_from_index, amount)?;
            let _new_balance_to = balance_add::<T>(&_account_index, &_token_to_index, amount)?;

            // for pool
            pool_change::<T>(&_pool_index, _direction == 0, amount, _direction != 0, amount)?;

            let op = Ops::Swap(sign, _account_index, _pool_index, amount, _direction, nonce);

            PendingReqMap::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &_token_from_index, _new_balance_from);
            balance_set(&_account_index, &_token_to_index, _new_balance_to);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(
                Event::<T>::SwapReq(
                    _req_id, sign, _account_index, _pool_index, amount, _direction, nonce
                )
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn pool_supply(
            origin,
            sign: Signature,
            account: T::AccountId,
            token_from: TokenIndex,
            token_to: TokenIndex,
            amount_from: Amount,
            amount_to: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let __token_from_index = token_from;
            let __token_to_index = token_to;
            let (_token_from_index, _token_from_amount, _token_to_index, _token_to_amount) =
                if __token_from_index < __token_to_index {
                    (__token_from_index, amount_from, __token_to_index, amount_to)
                } else {
                    (__token_to_index, amount_to, __token_from_index, amount_from)
                };

            let _pool_index = get_pool_index::<T>(&_token_from_index, &_token_to_index)?;
            let _req_id = req_id_get::<T>()?;
            let _new_nonce = nonce_check::<T>(&account, nonce)?;

            // for user account
            let _new_balance_from = balance_sub::<T>(&_account_index, &_token_from_index, _token_from_amount)?;
            let _new_balance_to = balance_sub::<T>(&_account_index, &_token_to_index, _token_to_amount)?;
            let _new_share = share_add::<T>(&_account_index, &_pool_index, _token_from_amount.checked_add(_token_to_amount).ok_or(Error::<T>::ShareOverflow)?)?;

            // for pool
            pool_change::<T>(&_pool_index, true, _token_from_amount, true, _token_to_amount)?;

            let op = Ops::PoolSupply(
                sign,
                _account_index, _pool_index, _token_from_amount, _token_to_amount, nonce
            );

            PendingReqMap::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &_token_from_index, _new_balance_from);
            balance_set(&_account_index, &_token_to_index, _new_balance_to);
            ShareMap::insert((&_account_index, _pool_index), _new_share);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(
                Event::<T>::PoolSupplyReq(
                    _req_id, sign, _account_index, _pool_index, _token_from_amount, _token_to_amount, nonce
                )
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn pool_retrieve(
            origin,
            sign: Signature,
            account: T::AccountId,
            token_from: TokenIndex,
            token_to: TokenIndex,
            amount_from: Amount,
            amount_to: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let __token_from_index = token_from;
            let __token_to_index = token_to;
            let (_token_from_index, _token_from_amount, _token_to_index, _token_to_amount) =
                if __token_from_index < __token_to_index {
                    (__token_from_index, amount_from, __token_to_index, amount_to)
                } else {
                    (__token_to_index, amount_to, __token_from_index, amount_from)
                };

            let _pool_index = get_pool_index::<T>(&_token_from_index, &_token_to_index)?;
            let _req_id = req_id_get::<T>()?;
            let _new_nonce = nonce_check::<T>(&account, nonce)?;

            // for user account
            let _new_balance_from = balance_add::<T>(&_account_index, &_token_from_index, _token_from_amount)?;
            let _new_balance_to = balance_add::<T>(&_account_index, &_token_to_index, _token_to_amount)?;
            let _new_share = share_sub::<T>(&_account_index, &_pool_index, _token_from_amount.checked_add(_token_to_amount).ok_or(Error::<T>::ShareNotEnough)?)?;

            // for pool
            pool_change::<T>(&_pool_index, false, _token_from_amount, false, _token_to_amount)?;

            let op = Ops::PoolRetrieve(
                sign,
                _account_index, _pool_index, _token_from_amount, _token_to_amount, nonce);

            PendingReqMap::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &_token_from_index, _new_balance_from);
            balance_set(&_account_index, &_token_to_index, _new_balance_to);
            ShareMap::insert((&_account_index, &_pool_index), _new_share);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(
                Event::<T>::PoolRetrieveReq(
                    _req_id, sign, _account_index, _pool_index, _token_from_amount, _token_to_amount, nonce,
                )
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn ack(
            origin,
            req_id: ReqId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            PendingReqMap::get(&req_id).ok_or(Error::<T>::InvalidReqId)?;

            let mut acks = AckMap::get(&req_id);

            if _who == T::ADMIN1::get() {
                acks = acks | ACK1;
            }

            if _who == T::ADMIN2::get() {
                acks = acks | ACK2;
            }

            AckMap::insert(&req_id, &acks);

            if acks == NACK {
                let l1txhash = DepositMap::get(&req_id);
                match l1txhash {
                    None => {},
                    Some(v) => {
                        L1TxMap::insert(v, DONE);
                    }
                };
                match PendingReqMap::get(&req_id) {
                    Some (req) => {
                        CompleteReqMap::insert(req_id, req);
                        PendingReqMap::remove(&req_id);
                    },
                    _ => {}
                };
            }

            Self::deposit_event(RawEvent::Ack(req_id, acks));
            return Ok(());
        }
    }
}
