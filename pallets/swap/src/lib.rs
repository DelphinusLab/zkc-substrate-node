#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use delphinus_crypto::Encode as BabyJubJubEncode;
use delphinus_crypto::{BabyJubjub, BabyJubjubField, BabyJubjubPoint, Curve, PrimeField, EDDSA};
use frame_support::traits::{Currency, ReservableCurrency};
use frame_support::{decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use num_bigint::{BigInt, Sign};
use sp_core::U256;

mod aux;
mod errors;
mod types;

pub use aux::*;
use errors::*;
use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
const NFT_TOKEN_INDEX: u32 = 1u32;

const OP_DEPOSIT: u8 = 0u8;
const OP_WITHDRAW: u8 = 1u8;
const OP_SWAP: u8 = 2u8;
const OP_RETRIEVE: u8 = 3u8;
const OP_SUPPLY: u8 = 4u8;
const OP_ADDPOOL: u8 = 5u8;
const OP_SETKEY: u8 = 6u8;
const OP_DEPOSIT_NFT: u8 = 7u8;
const OP_WITHDRAW_NFT: u8 = 8u8;
const OP_TRANSFER_NFT: u8 = 9u8;
const OP_BID_NFT: u8 = 10u8;
const OP_FINALIZE_NFT: u8 = 11u8;

decl_event!(
    pub enum Event<T>
    where
        Balance = BalanceOf<T>,
        BlockNumber = <T as frame_system::Config>::BlockNumber,
        AccountId = <T as frame_system::Config>::AccountId,
    {
        SetKey(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            ReserveU32,
            PublicKeyX,
            PublicKeyY,
        ),
        Deposit(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            TokenIndex,
            Amount,
            ReserveU256,
        ),
        Withdraw(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            TokenIndex,
            Amount,
            L1Account,
        ),
        Swap(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            PoolIndex,
            Reverse,
            Amount,
        ),
        PoolSupply(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            PoolIndex,
            Amount,
            Amount,
        ),
        PoolRetrieve(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            PoolIndex,
            Amount,
            Amount,
        ),
        AddPool(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            TokenIndex,
            TokenIndex,
            ReserveU256,
            ReserveU256,
            PoolIndex,
        ),
        DepositNFT(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            NFTId
        ),
        WithdrawNFT(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            NFTId,
            L1Account,
        ),
        TransferNFT(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex, // From
            AccountIndex, // To
            NFTId,
        ),
        BidNFT(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            AccountIndex,
            Amount,
            NFTId
        ),
        FinalizeNFT(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            AccountIndex,
            Amount,
            NFTId,
        ),
        Ack(ReqId, u8),
        Abort(ReqId),
        RewardFunds(AccountId, Balance, BlockNumber),
    }
);

decl_storage! {
    trait Store for Module<T: Config> as SimpleMap {
        pub AccountIndexCount: AccountIndex;
        pub AccountIndexMap get(fn account_index_map): map hasher(blake2_128_concat) T::AccountId => Option<AccountIndex>;

        pub PoolIndexCount: PoolIndex;
        pub PoolIndexMap get(fn pool_index_map): map hasher(blake2_128_concat) (TokenIndex, TokenIndex) => Option<PoolIndex>;

        pub NFTIDCount: NFTId;

        pub BalanceMap get(fn balance_map): map hasher(blake2_128_concat) (AccountIndex, TokenIndex) => Amount;
        pub ShareMap get(fn share_map): map hasher(blake2_128_concat) (AccountIndex, PoolIndex) => Amount;
        pub PoolMap get(fn pool_map): map hasher(blake2_128_concat) PoolIndex => Option<(TokenIndex, TokenIndex, Amount, Amount)>;

        /* Owner * bid * CurrentWinner */
        pub NFTMap get(fn nft_map): map hasher(blake2_128_concat) NFTId => (AccountIndex, Amount, Option<AccountIndex>);

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
            let who = ensure_signed(origin)?;
            let _r = T::Currency::deposit_creating(&account, reward);
            let now = <frame_system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::RewardFunds(who, reward, now));
            return Ok(());
        }

        #[weight = 0]
        pub fn set_key(origin, key: [u8; 32]) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            let key = BabyJubjubPoint::decode(&key).map_err(|_| Error::<T>::InvalidKey)?;

            let x = u256_from_bigint(&key.x.v);
            let y = u256_from_bigint(&key.y.v);

            let req_id = req_id_get::<T>()?;
            let account_index = create_account_index::<T>(&who)?;
            let nonce = NonceMap::<T>::get(&who);
            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let op = Ops::SetKey(U256::from(0u8), U256::from(0u8), U256::from(0u8), nonce, account_index, 0u32, x, y);

            PendingReqMap::insert(&req_id, op);
            KeyMap::insert(account_index, (x, y));
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(RawEvent::SetKey(req_id, U256::from(0u8), U256::from(0u8), U256::from(0u8), nonce, account_index, 0u32, x, y));
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
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&token_index_0.to_be_bytes());
            command[13..17].copy_from_slice(&token_index_1.to_be_bytes());
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
            let op = Ops::AddPool(sign.0, sign.1, sign.2, nonce, token_index_0, token_index_1, U256::from(0u8), U256::from(0u8), pool_index);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(Event::<T>::AddPool(
                req_id, sign.0, sign.1, sign.2, nonce,
                token_index_0, token_index_1, U256::from(0u8), U256::from(0u8), pool_index
            ));
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
            is_admin::<T>(&who)?;

            let who_account_index = get_account_index::<T>(&who)?;

            if token_index >= MAX_TOKEN_COUNT {
                return Err(Error::<T>::InvalidTokenIndex)?;
            }


            if L1TxMap::get(l1_tx_hash) != 0u8 {
                return Err(Error::<T>::L1TXExists)?;
            }

            if amount == U256::from(0u8) {
                return Err(Error::<T>::InvalidAmount)?;
            }

            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_DEPOSIT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&token_index.to_be_bytes());
            command[17..49].copy_from_slice(&amount.to_be_bytes());
            command[49..81].copy_from_slice(&l1_tx_hash.to_be_bytes());
            let sign = check_sign::<T>(who_account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            let new_balance_amount = balance_add::<T>(&account_index, &token_index, amount)?;

            let op = Ops::Deposit(sign.0, sign.1, sign.2, nonce, account_index, token_index, amount, U256::from(0u8));

            balance_set(&account_index, &token_index, new_balance_amount);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);
            DepositMap::insert(&req_id, l1_tx_hash);
            L1TxMap::insert(&l1_tx_hash, PENDING);

            Self::deposit_event(Event::<T>::Deposit(req_id, sign.0, sign.1, sign.2, nonce, account_index, token_index, amount, U256::from(0u8)));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn withdraw(
            origin,
            sign: [u8; 64],
            token_index: TokenIndex,
            amount: Amount,
            l1account: L1Account,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let req_id = req_id_get::<T>()?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;
            let new_balance = balance_sub::<T>(&account_index, &token_index, amount)?;
            l1account_check::<T>(l1account)?;

            let mut command = [0u8; 81];
            command[0] = OP_WITHDRAW;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&token_index.to_be_bytes());
            command[17..49].copy_from_slice(&amount.to_be_bytes());
            command[49..81].copy_from_slice(&l1account.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let op = Ops::Withdraw(sign.0, sign.1, sign.2, nonce, account_index, token_index, amount, l1account);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            balance_set(&account_index, &token_index, new_balance);
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(Event::<T>::Withdraw(
                req_id,
                sign.0, sign.1, sign.2, nonce, account_index, token_index, amount, l1account
            ));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn swap(
            origin,
            sign: [u8; 64],
            pool_index: PoolIndex,
            reverse: Reverse,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let req_id = req_id_get::<T>()?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let (token0, token1, _, _) = PoolMap::get(&pool_index).ok_or(Error::<T>::PoolNotExists)?;
            let (token0, token1) = if reverse == 0u8 { (token0, token1) } else { (token1, token0) };

            let mut command = [0u8; 81];
            command[0] = OP_SWAP;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&pool_index.to_be_bytes());
            command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
            command[49..81].copy_from_slice(&amount.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let new_balance_from = balance_sub::<T>(&account_index, &token0, amount)?;
            let new_balance_to = balance_add::<T>(&account_index, &token1, amount)?;

            pool_change::<T>(&pool_index, reverse == 0, amount, reverse != 0, amount)?;

            let op = Ops::Swap(sign.0, sign.1, sign.2, nonce, account_index, pool_index, reverse, amount);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            balance_set(&account_index, &token0, new_balance_from);
            balance_set(&account_index, &token1, new_balance_to);
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(
                Event::<T>::Swap(
                    req_id,
                    sign.0, sign.1, sign.2, nonce, account_index, pool_index, reverse, amount
                )
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn pool_supply(
            origin,
            sign: [u8; 64],
            pool_index: PoolIndex,
            amount0: Amount,
            amount1: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let (token0, token1, _, _) = PoolMap::get(&pool_index).ok_or(Error::<T>::PoolNotExists)?;

            let req_id = req_id_get::<T>()?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_SUPPLY;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&pool_index.to_be_bytes());
            command[17..49].copy_from_slice(&amount0.to_be_bytes());
            command[49..81].copy_from_slice(&amount1.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let new_balance_from = balance_sub::<T>(&account_index, &token0, amount0)?;
            let new_balance_to = balance_sub::<T>(&account_index, &token1, amount1)?;
            let new_share = share_add::<T>(&account_index, &pool_index, amount0.checked_add_on_bn128(amount1).ok_or(Error::<T>::ShareOverflow)?)?;

            pool_change::<T>(&pool_index, true, amount0, true, amount1)?;

            let op = Ops::PoolSupply(sign.0, sign.1, sign.2, nonce, account_index, pool_index, amount0, amount1);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            balance_set(&account_index, &token0, new_balance_from);
            balance_set(&account_index, &token1, new_balance_to);
            ShareMap::insert((&account_index, pool_index), new_share);
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(
                Event::<T>::PoolSupply(
                    req_id, sign.0, sign.1, sign.2, nonce, account_index, pool_index, amount0, amount1)
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn pool_retrieve(
            origin,
            sign: [u8; 64],
            pool_index: PoolIndex,
            amount0: Amount,
            amount1: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let (token0, token1, _, _) = PoolMap::get(&pool_index).ok_or(Error::<T>::PoolNotExists)?;

            let req_id = req_id_get::<T>()?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_RETRIEVE;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&pool_index.to_be_bytes());
            command[17..49].copy_from_slice(&amount0.to_be_bytes());
            command[49..81].copy_from_slice(&amount1.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            // for user account
            let new_balance_from = balance_add::<T>(&account_index, &token0, amount0)?;
            let new_balance_to = balance_add::<T>(&account_index, &token1, amount1)?;
            let new_share = share_sub::<T>(&account_index, &pool_index, amount0.checked_add_on_bn128(amount1).ok_or(Error::<T>::ShareNotEnough)?)?;

            // for pool
            pool_change::<T>(&pool_index, false, amount0, false, amount1)?;

            let op = Ops::PoolRetrieve(sign.0, sign.1, sign.2, nonce, account_index, pool_index, amount0, amount1);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            balance_set(&account_index, &token0, new_balance_from);
            balance_set(&account_index, &token1, new_balance_to);
            ShareMap::insert((&account_index, &pool_index), new_share);
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(
                Event::<T>::PoolRetrieve(
                    req_id, sign.0, sign.1, sign.2, nonce, account_index, pool_index, amount0, amount1
                )
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn deposit_nft(
            origin,
            sign: [u8; 64],
            account_index: AccountIndex,
            nft_id: NFTId,
            l1_tx_hash: L1TxHash,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let who_account_index = get_account_index::<T>(&who)?;

            if L1TxMap::get(l1_tx_hash) != 0u8 {
                return Err(Error::<T>::L1TXExists)?;
            }

            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_DEPOSIT_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&nft_id.to_be_bytes());
            command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());
            let sign = check_sign::<T>(who_account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            let _new_balance_amount = nft_add::<T>(&account_index, &nft_id)?;
            let op = Ops::DepositNFT(sign.0, sign.1, sign.2, nonce, account_index, nft_id);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);
            DepositMap::insert(&req_id, l1_tx_hash);
            L1TxMap::insert(&l1_tx_hash, PENDING);

            Self::deposit_event(Event::<T>::DepositNFT(req_id, sign.0, sign.1, sign.2, nonce, account_index, nft_id));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn withdraw_nft(
            origin,
            sign: [u8; 64],
            nft_id: NFTId,
            l1account: L1Account,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let req_id = req_id_get::<T>()?;
            l1account_check::<T>(l1account)?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_WITHDRAW_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&nft_id.to_be_bytes());
            command[17..49].copy_from_slice(&l1account.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let op = Ops::WithdrawNFT(sign.0, sign.1, sign.2, nonce, account_index, nft_id, l1account);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            nft_withdraw::<T>(&account_index, &nft_id)?;
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(Event::<T>::WithdrawNFT(
                req_id,
                sign.0, sign.1, sign.2, nonce, account_index, nft_id, l1account
            ));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn transfer_nft(
            origin,
            sign: [u8; 64],
            nft_id: NFTId,
            recipent: AccountIndex,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let req_id = req_id_get::<T>()?;
            if recipent >= AccountIndexCount::get() {
                return Err(Error::<T>::InvalidAccount)?;
            }
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_TRANSFER_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&recipent.to_be_bytes());
            command[17..21].copy_from_slice(&nft_id.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let op = Ops::TransferNFT(sign.0, sign.1, sign.2, nonce, account_index, recipent, nft_id);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            nft_transfer::<T>(&account_index, &recipent, &nft_id)?;
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(Event::<T>::TransferNFT(
                req_id,
                sign.0, sign.1, sign.2, nonce, account_index, recipent, nft_id
            ));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn bid_nft(
            origin,
            sign: [u8; 64],
            nft_id: NFTId,
            owner: AccountIndex,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let req_id = req_id_get::<T>()?;
            if amount == U256::from(0u8) {
                return Err(Error::<T>::InvalidAmount)?;
            }

            let _new_balance = balance_sub::<T>(&account_index, &NFT_TOKEN_INDEX, amount)?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_BID_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&owner.to_be_bytes());
            command[13..17].copy_from_slice(&account_index.to_be_bytes());
            command[17..49].copy_from_slice(&amount.to_be_bytes());
            command[49..53].copy_from_slice(&nft_id.to_be_bytes());

            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let op = Ops::BidNFT(sign.0, sign.1, sign.2, nonce, owner, account_index, amount, nft_id);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            nft_bid::<T>(&owner, &account_index, amount, &nft_id)?;
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(Event::<T>::BidNFT(
                req_id,
                sign.0, sign.1, sign.2, nonce, owner, account_index, amount, nft_id
            ));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn finalize_nft(
            origin,
            sign: [u8; 64],
            nft_id: NFTId,
            bidder: AccountIndex,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account = who;
            let account_index = get_account_index::<T>(&account)?;

            let req_id = req_id_get::<T>()?;
            let _new_balance = balance_add::<T>(&account_index, &NFT_TOKEN_INDEX, amount)?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_FINALIZE_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&bidder.to_be_bytes());
            command[17..49].copy_from_slice(&amount.to_be_bytes());
            command[49..53].copy_from_slice(&nft_id.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let op = Ops::FinalizeNFT(sign.0, sign.1, sign.2, nonce, account_index, bidder, amount, nft_id);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            nft_finalize::<T>(&account_index, &bidder, &nft_id)?;
            NonceMap::<T>::insert(&account, new_nonce);

            Self::deposit_event(Event::<T>::FinalizeNFT(
                req_id,
                sign.0, sign.1, sign.2, nonce, account_index, bidder, amount, nft_id
            ));

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
