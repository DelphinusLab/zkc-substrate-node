#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::traits::{Currency, ReservableCurrency};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_core::U256;

#[cfg(test)]
mod mock;

pub trait Config: frame_system::Config {
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type ADMIN1: Get<<Self as frame_system::Config>::AccountId>;
    type ADMIN2: Get<<Self as frame_system::Config>::AccountId>;
}

decl_error! {
    pub enum Error for Module<T: Config> {
        NoneValue,
        BalanceOverflow,
        BalanceNotEnough,
        LockedBalanceOverflow,
        LockedBalanceNotEnough,
        PoolBalanceNotEnough,
        PoolBalanceOverflow,
        ShareOverflow,
        ShareNotEnough,
        ReqIdOverflow,
        InvalidReqId,
        NotImplemented,
        NonceInconsistent,
        NonceOverflow,
        InvalidPool,
        AccountExists,
        AccountNotExists,
        AccountIndexOverflow,
        TokenExists,
        TokenNotExists,
        TokenIndexOverflow,
        NoAccess,
        PoolExists,
        PoolNotExists,
        PoolIndexOverflow,
        L1TXExists,
        InvalidTokenPair,
        InvalidTokenIndex,
    }
}

fn is_admin<T: Config>(who: &T::AccountId) -> Result<(), Error<T>> {
    if *who == T::ADMIN1::get() || *who == T::ADMIN2::get() {
        return Ok(());
    }

    return Err(Error::<T>::NoAccess);
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

type TokenAddr = U256;
type NonceId = U256;
type ReqId = U256;
type Amount = U256;

type L1Account = U256;
type L1TxHash = U256;

type AccountIndex = u32;
type TokenIndex = u32;
type PoolIndex = u32;

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        Balance = BalanceOf<T>,
        BlockNumber = <T as frame_system::Config>::BlockNumber,
    {
        Deposit(ReqId, AccountId, TokenAddr, Amount, NonceId, Amount),
        WithdrawReq(
            ReqId,
            AccountId,
            L1Account,
            TokenAddr,
            Amount,
            NonceId,
            Amount,
        ),
        SwapReq(
            ReqId,
            AccountId,
            TokenAddr,
            TokenAddr,
            Amount,
            NonceId,
            Amount,
            Amount,
            Amount,
            Amount,
        ),
        PoolSupplyReq(
            ReqId,
            AccountId,
            TokenAddr,
            TokenAddr,
            Amount,
            Amount,
            NonceId,
            Amount,
            Amount,
            Amount,
            Amount,
            Amount,
        ),
        PoolRetrieveReq(
            ReqId,
            AccountId,
            TokenAddr,
            TokenAddr,
            Amount,
            Amount,
            NonceId,
            Amount,
            Amount,
            Amount,
            Amount,
            Amount,
        ),
        Ack(ReqId, u8),
        Abort(ReqId),
        RewardFunds(AccountId, Balance, BlockNumber),
        AddTokenReq(ReqId, TokenAddr, TokenIndex),
        AddPoolReq(ReqId, TokenIndex, TokenIndex, PoolIndex),
    }
);

#[derive(Encode, Decode, Clone, PartialEq)]
pub enum Ops<T: frame_system::Config> {
    /// Input: account, token, amount, nonce
    /// OutPut: new_account_amount,
    Deposit(T::AccountId, TokenAddr, Amount, NonceId, Amount, L1TxHash),
    /// Input: account, l1account, token, amount, nonce
    /// OutPut: new_account_amount
    Withdraw(T::AccountId, L1Account, TokenAddr, Amount, NonceId, Amount),
    /// Input: account, from, to, amount, nonce
    /// OutPut: new_pool_amount_from, new_pool_amount_from, new_account_amount_from, new_account_amount_to,
    Swap(
        T::AccountId,
        TokenAddr,
        TokenAddr,
        Amount,
        NonceId,
        Amount,
        Amount,
        Amount,
        Amount,
    ),
    /// Input: account, from, to, amount_from, amount_to, nonce
    /// OutPut: new_pool_amount_from, new_pool_amount_from, new_account_amount_from, new_account_amount_to, new_share_amount
    PoolSupply(
        T::AccountId,
        TokenAddr,
        TokenAddr,
        Amount,
        Amount,
        NonceId,
        Amount,
        Amount,
        Amount,
        Amount,
        Amount,
    ),
    /// Input: account, from, to, amount, nonce
    /// OutPut: new_pool_amount_from, new_pool_amount_from, new_account_amount_from, new_account_amount_to, new_share_amount
    PoolRetrieve(
        T::AccountId,
        TokenAddr,
        TokenAddr,
        Amount,
        Amount,
        NonceId,
        Amount,
        Amount,
        Amount,
        Amount,
        Amount,
    ),
    /// Input: token_addr
    /// Output: token_index
    AddToken(TokenAddr, TokenIndex),
    /// Input: token_index_pair
    /// Output: pool_index
    AddPool(TokenIndex, TokenIndex, PoolIndex),
}

decl_storage! {
    trait Store for Module<T: Config> as SimpleMap {
        pub AccountIndexCount: AccountIndex;
        pub AccountIndexMap get(fn account_index_map): map hasher(blake2_128_concat) T::AccountId => Option<AccountIndex>;

        pub TokenIndexCount: TokenIndex;
        pub TokenIndexMap get(fn token_index_map): map hasher(blake2_128_concat) TokenAddr => Option<TokenIndex>;

        pub PoolIndexCount: PoolIndex;
        pub PoolIndexMap get(fn pool_index_map): map hasher(blake2_128_concat) (TokenIndex, TokenIndex) => Option<PoolIndex>;

        pub BalanceMap get(fn balance_map): map hasher(blake2_128_concat) (AccountIndex, TokenIndex) => Amount;
        pub ShareMap get(fn share_map): map hasher(blake2_128_concat) (AccountIndex, PoolIndex) => Amount;
        pub PoolMap get(fn pool_map): map hasher(blake2_128_concat) PoolIndex => (TokenIndex, TokenIndex, Amount, Amount);

        pub PendingReqMap get(fn pending_req_map): map hasher(blake2_128_concat) ReqId => Option<Ops<T>>;
        pub AckMap get(fn ack_map): map hasher(blake2_128_concat) ReqId => u8;
        pub ReqIndex get(fn req_index): ReqId;
        pub NonceMap get(fn nonce_map): map hasher(blake2_128_concat) T::AccountId => NonceId;
        pub DepositMap get(fn deposit_map): map hasher(blake2_128_concat) L1TxHash => u8;
    }
}

fn nonce_check<T: Config>(account: &T::AccountId, nonce: NonceId) -> Result<NonceId, Error<T>> {
    if nonce != NonceMap::<T>::get(account) {
        return Err(Error::<T>::NonceInconsistent);
    }

    let new_nonce = nonce
        .checked_add(U256::from(1))
        .ok_or(Error::<T>::NonceOverflow)?;
    return Ok(new_nonce);
}

/* ---- Account Index ---- */
fn get_account_index<T: Config>(account: &T::AccountId) -> Result<AccountIndex, Error<T>> {
    let account_index = AccountIndexMap::<T>::get(&account).ok_or(Error::<T>::AccountNotExists)?;
    return Ok(account_index);
}

fn create_account_index<T: Config>(account: &T::AccountId) -> Result<AccountIndex, Error<T>> {
    if get_account_index::<T>(account).is_ok() {
        return Err(Error::<T>::TokenExists);
    }

    let index = AccountIndexCount::get();
    if index >= MAX_ACCOUNT_COUNT {
        return Err(Error::<T>::AccountIndexOverflow);
    }
    AccountIndexCount::set(index + 1);
    AccountIndexMap::<T>::insert(account, index);
    return Ok(index);
}

fn get_or_create_account_index<T: Config>(
    account: &T::AccountId,
) -> Result<AccountIndex, Error<T>> {
    let account_index_opt = AccountIndexMap::<T>::get(&account);
    return match account_index_opt {
        None => create_account_index(account),
        Some(account_index) => Ok(account_index),
    };
}

/* ---- Token Index ---- */
fn get_token_index<T: Config>(token: &TokenAddr) -> Result<TokenIndex, Error<T>> {
    let token_index = TokenIndexMap::get(&token).ok_or(Error::<T>::TokenNotExists)?;
    return Ok(token_index);
}

fn create_token_index<T: Config>(token: &TokenAddr) -> Result<TokenIndex, Error<T>> {
    if get_token_index::<T>(token).is_ok() {
        return Err(Error::<T>::TokenExists);
    }

    let index = TokenIndexCount::get();
    if index >= MAX_TOKEN_COUNT {
        return Err(Error::<T>::TokenIndexOverflow);
    }
    TokenIndexCount::set(index + 1);
    TokenIndexMap::insert(token, index);
    return Ok(index);
}

/* ---- Pool Index ---- */
fn get_pool_index<T: Config>(
    token_src_index: &TokenIndex,
    token_dst_index: &TokenIndex,
) -> Result<TokenIndex, Error<T>> {
    let pool_index =
        PoolIndexMap::get((token_src_index, token_dst_index)).ok_or(Error::<T>::PoolNotExists)?;
    return Ok(pool_index);
}

fn create_pool_index<T: Config>(
    token_src_index: &TokenIndex,
    token_dst_index: &TokenIndex,
) -> Result<TokenIndex, Error<T>> {
    if get_pool_index::<T>(token_src_index, token_dst_index).is_ok() {
        return Err(Error::<T>::PoolExists);
    }

    let index = PoolIndexCount::get();
    if index >= MAX_POOL_COUNT {
        return Err(Error::<T>::PoolIndexOverflow);
    }
    PoolIndexCount::set(index + 1);
    PoolIndexMap::insert((token_src_index, token_dst_index), index);
    PoolMap::insert(
        index,
        (
            token_src_index.clone(),
            token_dst_index.clone(),
            U256::from(0),
            U256::from(0),
        ),
    );
    return Ok(index);
}

/* ---- Balance ---- */

fn balance_add<T: Config>(
    account_index: &AccountIndex,
    token_index: &TokenIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = BalanceMap::get((&account_index, token_index))
        .checked_add(amount)
        .ok_or(Error::<T>::BalanceOverflow)?;
    return Ok(new_amount);
}

fn balance_sub<T: Config>(
    account_index: &AccountIndex,
    token_index: &TokenIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = BalanceMap::get((&account_index, token_index))
        .checked_sub(amount)
        .ok_or(Error::<T>::BalanceNotEnough)?;
    return Ok(new_amount);
}

fn balance_set(account_index: &AccountIndex, token_index: &TokenIndex, amount: Amount) -> () {
    BalanceMap::insert((&account_index, token_index), amount);
}

/* ---- Pool ---- */
fn pool_change<T: Config>(
    pool_index: &PoolIndex,
    is_add_0: bool,
    change_0: Amount,
    is_add_1: bool,
    change_1: Amount,
) -> Result<(Amount, Amount), Error<T>> {
    let (token_index_0, token_index_1, amount_0, amount_1) = PoolMap::get(pool_index);
    let new_amount_0 = if is_add_0 {
        amount_0
            .checked_add(change_0)
            .ok_or(Error::<T>::PoolBalanceOverflow)?
    } else {
        amount_0
            .checked_sub(change_0)
            .ok_or(Error::<T>::PoolBalanceNotEnough)?
    };
    let new_amount_1 = if is_add_1 {
        amount_1
            .checked_add(change_1)
            .ok_or(Error::<T>::PoolBalanceOverflow)?
    } else {
        amount_1
            .checked_sub(change_1)
            .ok_or(Error::<T>::PoolBalanceNotEnough)?
    };
    PoolMap::insert(
        pool_index,
        (token_index_0, token_index_1, new_amount_0, new_amount_1),
    );
    return Ok((new_amount_0, new_amount_1));
}

/* ---- Share ---- */
fn share_add<T: Config>(
    account_index: &AccountIndex,
    pool_index: &PoolIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = ShareMap::get((account_index, pool_index))
        .checked_add(amount)
        .ok_or(Error::<T>::ShareOverflow)?;
    return Ok(new_amount);
}

fn share_sub<T: Config>(
    account_index: &AccountIndex,
    pool_index: &PoolIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = ShareMap::get((&account_index, pool_index))
        .checked_sub(amount)
        .ok_or(Error::<T>::ShareNotEnough)?;
    return Ok(new_amount);
}

fn req_id_get<T: Config>() -> Result<ReqId, Error<T>> {
    let req_id = ReqIndex::get()
        .checked_add(U256::from(1))
        .ok_or(Error::<T>::ReqIdOverflow)?;
    return Ok(req_id);
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// Awards the specified amount of funds to the specified account
        #[weight = 0]
        pub fn charge(origin, account: T::AccountId, reward: BalanceOf<T>) {
            let _who = ensure_signed(origin)?;
            let _r = T::Currency::deposit_creating(&account, reward);
            let now = <frame_system::Module<T>>::block_number();
            Self::deposit_event(RawEvent::RewardFunds(account, reward, now));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn add_token(
            origin,
            token_address: TokenAddr,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _ = is_admin::<T>(&_who)?;

            let _new_nonce = nonce_check::<T>(&_who, nonce)?;
            let _req_id = req_id_get::<T>()?;
            let _token_index = create_token_index::<T>(&token_address)?;
            let op = Ops::AddToken(token_address, _token_index);

            PendingReqMap::<T>::insert(&_req_id, op);
            ReqIndex::put(_req_id);
            NonceMap::<T>::insert(&_who, _new_nonce);

            Self::deposit_event(Event::<T>::AddTokenReq(_req_id, token_address, _token_index));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn add_pool(
            origin,
            token_index_0: TokenIndex,
            token_index_1: TokenIndex,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _ = is_admin::<T>(&_who)?;

            let _new_nonce = nonce_check::<T>(&_who, nonce)?;
            let _req_id = req_id_get::<T>()?;

            if token_index_0 >= TokenIndexCount::get() || token_index_0 >= TokenIndexCount::get() {
                return Err(Error::<T>::InvalidTokenIndex)?;
            }

            if token_index_0 == token_index_1 {
                return Err(Error::<T>::InvalidTokenPair)?;
            }

            let (_token_index_0, _token_index_1) =
                if token_index_0 < token_index_1 {
                    (token_index_0, token_index_1)
                } else {
                    (token_index_1, token_index_0)
                };

            let _pool_index = create_pool_index::<T>(&_token_index_0, &_token_index_1)?;
            let op = Ops::AddPool(token_index_0, token_index_1, _pool_index);

            PendingReqMap::<T>::insert(&_req_id, op);
            ReqIndex::put(_req_id);
            NonceMap::<T>::insert(&_who, _new_nonce);

            Self::deposit_event(Event::<T>::AddPoolReq(_req_id, token_index_0, token_index_1, _pool_index));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn deposit(
            origin,
            account: T::AccountId,
            token_address: TokenAddr,
            amount: Amount,
            nonce: NonceId,
            l1_tx_hash: U256
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let _token_index = get_token_index::<T>(&token_address)?;

            if DepositMap::get(l1_tx_hash) != 0u8 {
                return Err(Error::<T>::L1TXExists)?;
            }

            let _new_nonce = nonce_check::<T>(&_who, nonce)?;
            let _req_id = req_id_get::<T>()?;
            let _ = get_or_create_account_index::<T>(&account);

            let _new_balance_amount = balance_add::<T>(&_account_index, &_token_index, amount)?;
            let op = Ops::Deposit(account.clone(), token_address, amount, nonce, _new_balance_amount.clone(), l1_tx_hash.clone());

            balance_set(&_account_index, &_token_index, _new_balance_amount);
            PendingReqMap::<T>::insert(&_req_id, op);
            ReqIndex::put(_req_id);
            NonceMap::<T>::insert(&_who, _new_nonce);
            DepositMap::insert(&l1_tx_hash, PENDING);

            Self::deposit_event(Event::<T>::Deposit(_req_id, account, token_address, amount, nonce, _new_balance_amount));
            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn withdraw(
            origin,
            account: T::AccountId,
            l1account: L1Account,
            token_address: TokenAddr,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let _token_index = get_token_index::<T>(&token_address)?;

            let _req_id = req_id_get::<T>()?;
            let _new_nonce = nonce_check::<T>(&account, nonce)?;
            let _new_balance = balance_sub::<T>(&_account_index, &_token_index, amount)?;

            let op = Ops::Withdraw(account.clone(), l1account, token_address, amount, nonce, _new_balance);
            PendingReqMap::<T>::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &_token_index, _new_balance);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(Event::<T>::WithdrawReq(_req_id, account, l1account, token_address, amount, nonce, _new_balance));

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn swap(
            origin,
            account: T::AccountId,
            token_from: TokenAddr,
            token_to: TokenAddr,
            amount: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let _token_from_index = get_token_index::<T>(&token_from)?;
            let _token_to_index = get_token_index::<T>(&token_to)?;
            let _direction = _token_from_index < _token_to_index;
            let _pool_index =
                if _direction {
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
            let (_0, _1) =pool_change::<T>(&_pool_index, _direction, amount, !_direction, amount)?;
            let (new_pool_balance_from, new_pool_balance_to) =
                if _direction {
                    (_0, _1)
                } else {
                    (_1, _0)
                };

            let op = Ops::Swap(account.clone(), token_from, token_to, amount, nonce,
                new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to);

            PendingReqMap::<T>::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &_token_from_index, _new_balance_from);
            balance_set(&_account_index, &_token_to_index, _new_balance_to);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(
                Event::<T>::SwapReq(
                    _req_id, account.clone(), token_from, token_to, amount, nonce,
                    new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to
                )
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn pool_supply(
            origin,
            account: T::AccountId,
            token_from: TokenAddr,
            token_to: TokenAddr,
            amount_from: Amount,
            amount_to: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let _token_from_index = get_token_index::<T>(&token_from)?;
            let _token_to_index = get_token_index::<T>(&token_to)?;
            let _direction = _token_from_index < _token_to_index;
            let _pool_index =
                if _direction {
                    get_pool_index::<T>(&_token_from_index, &_token_to_index)?
                } else {
                    get_pool_index::<T>(&_token_to_index, &_token_from_index)?
                };

            let _req_id = req_id_get::<T>()?;
            let _new_nonce = nonce_check::<T>(&account, nonce)?;

            // for user account
            let _new_balance_from = balance_sub::<T>(&_account_index, &_token_from_index, amount_from)?;
            let _new_balance_to = balance_sub::<T>(&_account_index, &_token_to_index, amount_to)?;
            let _new_share = share_add::<T>(&_account_index, &_pool_index, amount_from.checked_add(amount_to).ok_or(Error::<T>::ShareOverflow)?)?;

            // for pool
            let (_0, _1) =pool_change::<T>(&_pool_index, true, amount_to, true, amount_from)?;
            let (new_pool_balance_from, new_pool_balance_to) =
                if _direction {
                    (_0, _1)
                } else {
                    (_1, _0)
                };


            let op = Ops::PoolSupply(
                account.clone(), token_from, token_to, amount_from, amount_to, nonce,
                new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to, _new_share);

            PendingReqMap::<T>::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &_token_from_index, _new_balance_from);
            balance_set(&_account_index, &_token_to_index, _new_balance_to);
            ShareMap::insert((&_account_index, _pool_index), _new_share);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(
                Event::<T>::PoolSupplyReq(
                    _req_id, account, token_from, token_to, amount_from, amount_to, nonce,
                    new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to, _new_share
                )
            );

            return Ok(());
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn pool_retrieve(
            origin,
            account: T::AccountId,
            token_from: TokenAddr,
            token_to: TokenAddr,
            amount_from: Amount,
            amount_to: Amount,
            nonce: NonceId
        ) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            let _account_index = get_account_index::<T>(&account)?;
            let _token_from_index = get_token_index::<T>(&token_from)?;
            let _token_to_index = get_token_index::<T>(&token_to)?;
            let _direction = _token_from_index < _token_to_index;
            let _pool_index =
                if _direction {
                    get_pool_index::<T>(&_token_from_index, &_token_to_index)?
                } else {
                    get_pool_index::<T>(&_token_to_index, &_token_from_index)?
                };

            let _req_id = req_id_get::<T>()?;
            let _new_nonce = nonce_check::<T>(&account, nonce)?;

            // for user account
            let _new_balance_from = balance_add::<T>(&_account_index, &_token_from_index, amount_from)?;
            let _new_balance_to = balance_add::<T>(&_account_index, &_token_to_index, amount_to)?;
            let _new_share = share_sub::<T>(&_account_index, &_pool_index, amount_from.checked_add(amount_to).ok_or(Error::<T>::ShareNotEnough)?)?;

            // for pool
            let (_0, _1) =pool_change::<T>(&_pool_index, false, amount_to, false, amount_from)?;
            let (new_pool_balance_from, new_pool_balance_to) =
                if _direction {
                    (_0, _1)
                } else {
                    (_1, _0)
                };

            let op = Ops::PoolRetrieve(
                account.clone(), token_from, token_to, amount_from, amount_to, nonce,
                new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to, _new_share);

            PendingReqMap::<T>::insert(&_req_id, op);
            ReqIndex::put(_req_id);

            balance_set(&_account_index, &_token_from_index, _new_balance_from);
            balance_set(&_account_index, &_token_to_index, _new_balance_to);
            ShareMap::insert((&_account_index, &_pool_index), _new_share);
            NonceMap::<T>::insert(&account, _new_nonce);

            Self::deposit_event(
                Event::<T>::PoolRetrieveReq(
                    _req_id, account, token_from, token_to, amount_from, amount_to, nonce,
                    new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to, _new_share
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
            let mut acks = AckMap::get(&req_id);

            if _who == T::ADMIN1::get() {
                acks = acks | ACK1;
            }

            if _who == T::ADMIN2::get() {
                acks = acks | ACK2;
            }

            AckMap::insert(&req_id, &acks);

            if acks == NACK {
                let op = PendingReqMap::<T>::get(&req_id).ok_or(Error::<T>::InvalidReqId)?;
                match op {
                    Ops::<T>::Deposit(_, _, _, _, _, l1_tx_hash) => DepositMap::insert(l1_tx_hash, DONE),
                    _ => {}
                }
                PendingReqMap::<T>::remove(&req_id);
            }

            Self::deposit_event(RawEvent::Ack(req_id, acks));
            return Ok(());
        }
    }
}
