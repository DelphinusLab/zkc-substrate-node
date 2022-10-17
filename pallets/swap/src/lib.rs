#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use delphinus_crypto::Encode as BabyJubJubEncode;
use delphinus_crypto::{BabyJubjub, BabyJubjubField, BabyJubjubPoint, Curve, PrimeField, EDDSA};
use frame_support::traits::{Currency, ReservableCurrency};
use frame_support::{decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_support::traits::Vec;
use num_bigint::{BigInt, Sign};
use sp_core::U256;

mod aux;
mod errors;
mod types;

use aux::*;
use errors::*;
use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type AckAdmins: Get<Vec<<Self as frame_system::Config>::AccountId>>;
}

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Initial 1 / sharePrice is 10 ^ 15
const ORDER_OF_MAGNITUDE: usize = 15usize;
const PENDING: u8 = 1u8;
const DONE: u8 = 2u8;

const MAX_ACCOUNT_COUNT: u32 = 1u32 << 20;
const MAX_NFTINDEX_COUNT: u32 = 1u32 << 20;
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
            AccountIndex
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
            AccountIndex
        ),
        DepositNFT(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            NFTId,
            AccountIndex
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
            NFTId,
            Amount
        ),
        FinalizeNFT(
            ReqId,
            SignatureRX,
            SignatureRY,
            SignatureS,
            NonceId,
            AccountIndex,
            NFTId
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
        pub PoolMap get(fn pool_map): map hasher(blake2_128_concat) PoolIndex => Option<(TokenIndex, TokenIndex, Amount, Amount, Amount)>;

        /* Owner * bid * CurrentWinner */
        pub NFTMap get(fn nft_map): map hasher(blake2_128_concat) NFTId => (AccountIndex, Amount, Option<AccountIndex>);

        pub PendingReqMap get(fn pending_req_map): map hasher(blake2_128_concat) ReqId => Option<Ops>;
        pub CompleteReqMap get(fn complete_req_map): map hasher(blake2_128_concat) ReqId => Option<Ops>;
        pub AckMap get(fn ack_map): map hasher(blake2_128_concat) ReqId => u8;
        pub ReqIndex get(fn req_index): ReqId;
        pub CompleteReqIndex get(fn complete_req_index): ReqId;
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
        pub fn charge(origin,
            account: T::AccountId,
            reward: BalanceOf<T>,
            l1_tx_hash: L1TxHash,
        ) {
            let who = ensure_signed(origin)?;
            let _r = T::Currency::deposit_creating(&account, reward);
            let now = <frame_system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::RewardFunds(who, reward, now));
            L1TxMap::insert(&l1_tx_hash, DONE);
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

            let op = Ops::SetKey(U256::from(0), U256::from(0), U256::from(0), nonce, account_index, 0u32, x, y);

            PendingReqMap::insert(&req_id, op);
            KeyMap::insert(account_index, (x, y));
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(RawEvent::SetKey(req_id, U256::from(0), U256::from(0), U256::from(0), nonce, account_index, 0u32, x, y));
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
            let op = Ops::AddPool(sign.0, sign.1, sign.2, nonce, token_index_0, token_index_1, U256::from(0), U256::from(0), pool_index, who_account_index);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(Event::<T>::AddPool(
                req_id, sign.0, sign.1, sign.2, nonce,
                token_index_0, token_index_1, U256::from(0), U256::from(0),
                pool_index, who_account_index
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

            if account_index >= AccountIndexCount::get() {
                return Err(Error::<T>::InvalidAccount)?;
            }

            if L1TxMap::get(l1_tx_hash) != 0u8 {
                return Err(Error::<T>::L1TXExists)?;
            }

            amount.valid_on_circuit().ok_or(Error::<T>::InvalidAmount)?;

            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_DEPOSIT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&token_index.to_be_bytes());
            command[17..49].copy_from_slice(&amount.to_be_bytes());
            // command[49..81] is reserved. The l1_tx_hash exceeds field limits, so not in signature.
            let sign = check_sign::<T>(who_account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            let new_balance_amount = balance_add::<T>(&account_index, &token_index, amount)?;

            let op = Ops::Deposit(sign.0, sign.1, sign.2, nonce, account_index, token_index, amount, U256::from(0), who_account_index);

            balance_set(&account_index, &token_index, new_balance_amount);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);
            DepositMap::insert(&req_id, l1_tx_hash);
            L1TxMap::insert(&l1_tx_hash, PENDING);

            Self::deposit_event(Event::<T>::Deposit(req_id, sign.0, sign.1, sign.2, nonce, account_index, token_index, amount, U256::from(0), who_account_index));
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

            if token_index >= MAX_TOKEN_COUNT {
                return Err(Error::<T>::InvalidTokenIndex)?;
            }

            amount.valid_on_circuit().ok_or(Error::<T>::InvalidAmount)?;

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

            if amount == U256::from(0) {
                return Err(Error::<T>::InvalidAmount)?;
            }
            valid_pool_amount(amount).ok_or(Error::<T>::InvalidAmount)?;

            let req_id = req_id_get::<T>()?;
            let new_nonce = nonce_check::<T>(&account, nonce)?;

            let ((token_input, amount_input), (token_output, amount_output)) = {
                let (token0, token1, amount0, amount1, _) = PoolMap::get(&pool_index).ok_or(Error::<T>::PoolNotExists)?;
                if reverse == 0u8 {
                    ((token0, amount0), (token1, amount1))
                } else {
                    ((token1, amount1), (token0, amount0))
                }
            };
            non_zero_pool_amount(amount_output).ok_or(Error::<T>::PoolBalanceNotEnough)?;

            let mut command = [0u8; 81];
            command[0] = OP_SWAP;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&pool_index.to_be_bytes());
            command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
            command[49..81].copy_from_slice(&amount.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let result_amount = calculate_swap_result_amount::<T>(amount_input, amount_output, amount)?;

            let new_balance_input = balance_sub::<T>(&account_index, &token_input, amount)?;
            let new_balance_output = balance_add::<T>(&account_index, &token_output, result_amount)?;

            pool_change::<T>(&pool_index, reverse == 0, if reverse == 0 {amount} else {result_amount}, reverse != 0, if reverse == 0 {result_amount} else {amount})?;

            let op = Ops::Swap(sign.0, sign.1, sign.2, nonce, account_index, pool_index, reverse, amount);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            balance_set(&account_index, &token_input, new_balance_input);
            balance_set(&account_index, &token_output, new_balance_output);
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

            valid_pool_amount(amount0).ok_or(Error::<T>::InvalidAmount)?;
            valid_pool_amount(amount1).ok_or(Error::<T>::InvalidAmount)?;
            if (amount0 * amount1) == U256::from(0) {
                return Err(Error::<T>::InvalidAmount)?;
            }

            let (token0, token1, liq0, liq1, _) = PoolMap::get(&pool_index).ok_or(Error::<T>::PoolNotExists)?;
            valid_input_y_amount(liq0, liq1, amount0, amount1, true).ok_or(Error::<T>::InvalidAmountRatio)?;

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

            let amount1_to_pool = if is_pool_empty(&pool_index) {
                amount1
            } else {
                calculate_amount1_to_pool::<T>(&pool_index, amount0, true)?
            };
            let new_balance_0 = balance_sub::<T>(&account_index, &token0, amount0)?;
            let new_balance_1 = balance_sub::<T>(&account_index, &token1, amount1_to_pool)?;
            let share_change = get_share_change::<T>(&pool_index, amount0, true)?;
            let new_share = share_add::<T>(&account_index, &pool_index, share_change)?;

            pool_change_with_share::<T>(&pool_index, true, amount0, true, amount1_to_pool, share_change)?;

            let op = Ops::PoolSupply(sign.0, sign.1, sign.2, nonce, account_index, pool_index, amount0, amount1);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            balance_set(&account_index, &token0, new_balance_0);
            balance_set(&account_index, &token1, new_balance_1);
            ShareMap::insert((&account_index, &pool_index), new_share);
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

            valid_pool_amount(amount0).ok_or(Error::<T>::InvalidAmount)?;
            valid_pool_amount(amount1).ok_or(Error::<T>::InvalidAmount)?;

            let (token0, token1, liq0, liq1, _) = PoolMap::get(&pool_index).ok_or(Error::<T>::PoolNotExists)?;
            valid_input_y_amount(liq0, liq1, amount0, amount1, false).ok_or(Error::<T>::InvalidAmountRatio)?;

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
            let amount1_to_pool = calculate_amount1_to_pool::<T>(&pool_index, amount0, false)?;
            let new_balance_0 = balance_add::<T>(&account_index, &token0, amount0)?;
            let new_balance_1 = balance_add::<T>(&account_index, &token1, amount1_to_pool)?;
            let share_change = get_share_change::<T>(&pool_index, amount0, false)?;
            let new_share = share_sub::<T>(&account_index, &pool_index, share_change)?;

            // for pool
            pool_change_with_share::<T>(&pool_index, false, amount0, false, amount1_to_pool, share_change)?;

            let op = Ops::PoolRetrieve(sign.0, sign.1, sign.2, nonce, account_index, pool_index, amount0, amount1);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);

            balance_set(&account_index, &token0, new_balance_0);
            balance_set(&account_index, &token1, new_balance_1);
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
            is_admin::<T>(&who)?;

            let caller_account_index = get_account_index::<T>(&who)?;

            validation_account_index::<T>(account_index)?;

            validation_nft_index::<T>(nft_id)?;

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
            let sign = check_sign::<T>(caller_account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            nft_add::<T>(&account_index, &nft_id)?;
            let op = Ops::DepositNFT(sign.0, sign.1, sign.2, nonce, account_index, nft_id, caller_account_index);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);
            DepositMap::insert(&req_id, l1_tx_hash);
            L1TxMap::insert(&l1_tx_hash, PENDING);

            Self::deposit_event(Event::<T>::DepositNFT(req_id, sign.0, sign.1, sign.2, nonce, account_index, nft_id, caller_account_index));

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
            let account_index = get_account_index::<T>(&who)?;

            validation_nft_index::<T>(nft_id)?;

            l1account_check::<T>(l1account)?;

            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_WITHDRAW_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&nft_id.to_be_bytes());
            command[17..49].copy_from_slice(&l1account.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            nft_withdraw::<T>(&account_index, &nft_id)?;
            let op = Ops::WithdrawNFT(sign.0, sign.1, sign.2, nonce, account_index, nft_id, l1account);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(Event::<T>::WithdrawNFT(req_id, sign.0, sign.1, sign.2, nonce, account_index, nft_id, l1account));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn transfer_nft(
            origin,
            sign: [u8; 64],
            nft_id: NFTId,
            recipient: AccountIndex,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account_index = get_account_index::<T>(&who)?;

            validation_nft_index::<T>(nft_id)?;

            validation_account_index::<T>(recipient)?;

            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut pad_recipient = [0u8; 32];
            pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
            let mut command = [0u8; 81];
            command[0] = OP_TRANSFER_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&nft_id.to_be_bytes());
            command[17..49].copy_from_slice(&pad_recipient);
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            nft_transfer::<T>(&account_index, &recipient, &nft_id)?;
            let op = Ops::TransferNFT(sign.0, sign.1, sign.2, nonce, account_index, nft_id, recipient);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(Event::<T>::TransferNFT(req_id, sign.0, sign.1, sign.2, nonce, account_index, recipient, nft_id));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn bid_nft(
            origin,
            sign: [u8; 64],
            nft_id: NFTId,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account_index = get_account_index::<T>(&who)?;

            validation_nft_index::<T>(nft_id)?;

            amount.valid_on_circuit().ok_or(Error::<T>::InvalidAmount)?;
            let nft = NFTMap::get(&nft_id);
            if amount <= nft.1 {
                return Err(Error::<T>::InvalidAmount)?;
            }

            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_BID_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&nft_id.to_be_bytes());
            command[17..49].copy_from_slice(&amount.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            nft_bid::<T>(&account_index, amount, &nft_id)?;
            let op = Ops::BidNFT(sign.0, sign.1, sign.2, nonce, account_index, nft_id, amount);
            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(Event::<T>::BidNFT(req_id, sign.0, sign.1, sign.2, nonce, account_index, nft_id, amount));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn finalize_nft(
            origin,
            sign: [u8; 64],
            nft_id: NFTId,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let account_index = get_account_index::<T>(&who)?;

            validation_nft_index::<T>(nft_id)?;

            let new_nonce = nonce_check::<T>(&who, nonce)?;

            let mut command = [0u8; 81];
            command[0] = OP_FINALIZE_NFT;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&nft_id.to_be_bytes());
            let sign = check_sign::<T>(account_index, &command, &sign)?;

            let req_id = req_id_get::<T>()?;

            nft_finalize::<T>(&account_index, &nft_id)?;
            let op = Ops::FinalizeNFT(sign.0, sign.1, sign.2, nonce, account_index, nft_id);

            PendingReqMap::insert(&req_id, op);
            ReqIndex::put(req_id);
            NonceMap::<T>::insert(&who, new_nonce);

            Self::deposit_event(Event::<T>::FinalizeNFT(req_id, sign.0, sign.1, sign.2, nonce, account_index, nft_id));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn ack(
            origin,
            req_id_start: ReqId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            let nack = (1u8 << T::AckAdmins::get().len()) - 1;

            let ack = T::AckAdmins::get().iter().position(|x| x.clone() == _who).ok_or(Error::<T>::NoAccess)?;
            let ack_bits = 1u8 << ack;

            let batch_size = 10 as u32;

            for i in 0..batch_size {
                let req_id = req_id_start + U256::from(i + 1);

                PendingReqMap::get(&req_id).ok_or(Error::<T>::InvalidReqId)?;

                let acks = AckMap::get(&req_id) | ack_bits;

                AckMap::insert(&req_id, &acks);

                if acks == nack {
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
            }

            CompleteReqIndex::set(req_id_start + U256::from(10 as u32));
            Self::deposit_event(RawEvent::Ack(req_id_start, ack_bits));
            return Ok(());
        }
    }
}
