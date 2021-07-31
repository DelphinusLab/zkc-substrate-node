#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use codec::{ Encode, Decode };
use sp_core::U256;

#[cfg(test)]
mod mock;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type ADMIN1: Get<<Self as frame_system::Config>::AccountId>;
	type ADMIN2: Get<<Self as frame_system::Config>::AccountId>;
}

const ACK1: u8 = 1u8;
const ACK2: u8 = 2u8;
const NACK: u8 = ACK1 | ACK2;
const PENDING: u8 = 1u8;
const DONE: u8 = 2u8;

type TokenAddr = U256;
type NonceId = U256;
type ReqId = U256;
type L1Account = U256;
type Amount = U256;
type L1TxHash = U256;

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		Deposit(ReqId, AccountId, TokenAddr, Amount, NonceId, Amount),
		WithdrawReq(ReqId, AccountId, L1Account, TokenAddr, Amount, NonceId, Amount),
		SwapReq(ReqId, AccountId, TokenAddr, TokenAddr, Amount, NonceId, Amount, Amount, Amount, Amount),
		PoolSupplyReq(ReqId, AccountId, TokenAddr, TokenAddr, Amount, Amount, NonceId, Amount, Amount, Amount, Amount, Amount),
		PoolRetrieveReq(ReqId, AccountId, TokenAddr, TokenAddr, Amount, Amount, NonceId, Amount, Amount, Amount, Amount, Amount),
		Ack(ReqId, u8),
		Abort(ReqId),
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
	Swap(T::AccountId, TokenAddr, TokenAddr, Amount, NonceId, Amount, Amount, Amount, Amount),
	/// Input: account, from, to, amount_from, amount_to, nonce
	/// OutPut: new_pool_amount_from, new_pool_amount_from, new_account_amount_from, new_account_amount_to, new_share_amount
	PoolSupply(T::AccountId, TokenAddr, TokenAddr, Amount, Amount, NonceId, Amount, Amount, Amount, Amount, Amount),
	/// Input: account, from, to, amount, nonce
	/// OutPut: new_pool_amount_from, new_pool_amount_from, new_account_amount_from, new_account_amount_to, new_share_amount
	PoolRetrieve(T::AccountId, TokenAddr, TokenAddr, Amount, Amount, NonceId, Amount, Amount, Amount, Amount, Amount),
}

decl_storage! {
	trait Store for Module<T: Config> as SimpleMap {
		pub BalanceMap get(fn balance_map): map hasher(blake2_128_concat) (T::AccountId, TokenAddr) => Amount;
		pub ShareMap get(fn share_map): map hasher(blake2_128_concat) (T::AccountId, (TokenAddr, TokenAddr)) => Amount;
		pub PoolMap get(fn pool_map): map hasher(blake2_128_concat) (TokenAddr, TokenAddr) => (Amount, Amount);
		pub PendingReqMap get(fn pending_req_map): map hasher(blake2_128_concat) ReqId => Option<Ops<T>>;
		pub AckMap get(fn ack_map): map hasher(blake2_128_concat) ReqId => u8;
		pub ReqIndex get(fn req_index): ReqId;
		pub NonceMap get(fn nonce_map): map hasher(blake2_128_concat) T::AccountId => NonceId;
		pub DepositMap get(fn deposit_map): map hasher(blake2_128_concat) L1TxHash => u8;
	}
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
	}
}

fn nonce_check<T: Config>(account: &T::AccountId, nonce: NonceId) -> Result<NonceId, Error::<T>> {
	if nonce != NonceMap::<T>::get(account) {
		return Err(Error::<T>::NonceInconsistent);
	}

	let new_nonce = nonce.checked_add(U256::from(1)).ok_or(Error::<T>::NonceOverflow)?;
	return Ok(new_nonce);
}

fn balance_add<T: Config>(account: &T::AccountId, token_address: TokenAddr, amount: Amount) -> Result<Amount, Error::<T>> {
	let new_amount = BalanceMap::<T>::get((&account, &token_address)).checked_add(amount).ok_or(Error::<T>::BalanceOverflow)?;
	return Ok(new_amount);
}

fn balance_sub<T: Config>(account: &T::AccountId, token_address: TokenAddr, amount: Amount) -> Result<Amount, Error::<T>> {
	let new_amount = BalanceMap::<T>::get((&account, &token_address)).checked_sub(amount).ok_or(Error::<T>::BalanceNotEnough)?;
	return Ok(new_amount);
}

fn share_add<T: Config>(account: &T::AccountId, pool_index: &(TokenAddr, TokenAddr), amount: Amount) -> Result<Amount, Error::<T>> {
	let new_amount = ShareMap::<T>::get((&account, &pool_index)).checked_add(amount).ok_or(Error::<T>::ShareOverflow)?;
	return Ok(new_amount);
}

fn share_sub<T: Config>(account: &T::AccountId, pool_index: &(TokenAddr, TokenAddr), amount: Amount) -> Result<Amount, Error::<T>> {
	let new_amount = ShareMap::<T>::get((&account, &pool_index)).checked_sub(amount).ok_or(Error::<T>::ShareNotEnough)?;
	return Ok(new_amount);
}

fn req_id_get<T: Config>() -> Result<ReqId, Error::<T>> {
	let req_id = ReqIndex::get().checked_add(U256::from(1)).ok_or(Error::<T>::ReqIdOverflow)?;
	return Ok(req_id);
}

fn pool_index<T: Config>(token_from: &TokenAddr, token_to: &TokenAddr) -> Result<(TokenAddr, TokenAddr), Error::<T>> {
	if token_from > token_to {
		return Ok((token_to.clone(), token_from.clone()));
	} else if token_from < token_to {
		return Ok((token_from.clone(), token_to.clone()));
	}else {
		return Err(Error::<T>::InvalidPool)
	};
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

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

			let _new_nonce = nonce_check::<T>(&_who, nonce)?;
			let _req_id = req_id_get::<T>()?;

			let _new_balance_amount = balance_add::<T>(&account, token_address, amount)?;

			let op = Ops::Deposit(account.clone(), token_address, amount, nonce, _new_balance_amount.clone(), l1_tx_hash.clone());
			PendingReqMap::<T>::insert(&_req_id, op);
			BalanceMap::<T>::insert((&account, &token_address), _new_balance_amount);
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

			let _req_id = req_id_get::<T>()?;
			let _new_nonce = nonce_check::<T>(&account, nonce)?;
			let _new_balance = balance_sub::<T>(&account, token_address, amount)?;

			let op = Ops::Withdraw(account.clone(), l1account, token_address, amount, nonce, _new_balance);
			PendingReqMap::<T>::insert(&_req_id, op);
			ReqIndex::put(_req_id);

			BalanceMap::<T>::insert((&account, token_address), _new_balance);
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

			let _req_id = req_id_get::<T>()?;
			let _new_nonce = nonce_check::<T>(&account, nonce)?;
			let _pool_index = pool_index::<T>(&token_from, &token_to)?;

			// for user account
			let _new_balance_from = balance_sub::<T>(&account, token_from, amount)?;
			let _new_balance_to = balance_add::<T>(&account, token_to, amount)?;

			// for pool
			let (new_pool_balance_from, new_pool_balance_to) =
				if token_from > token_to {
					let (pool_balance_to, pool_balance_from) = PoolMap::get(_pool_index);
					let new_pool_balance_from = pool_balance_from.checked_add(amount).ok_or(Error::<T>::PoolBalanceOverflow)?;
					let new_pool_balance_to = pool_balance_to.checked_sub(amount).ok_or(Error::<T>::PoolBalanceNotEnough)?;
					PoolMap::insert(_pool_index, (new_pool_balance_to, new_pool_balance_from));
					(new_pool_balance_to, new_pool_balance_from)
				} else if token_from < token_to {
					let (pool_balance_from, pool_balance_to) = PoolMap::get(_pool_index);
					let new_pool_balance_from = pool_balance_from.checked_add(amount).ok_or(Error::<T>::PoolBalanceOverflow)?;
					let new_pool_balance_to = pool_balance_to.checked_sub(amount).ok_or(Error::<T>::PoolBalanceNotEnough)?;
					PoolMap::insert(_pool_index, (new_pool_balance_from, new_pool_balance_to));
					(new_pool_balance_from, new_pool_balance_to)
				} else {
					Err(Error::<T>::InvalidPool)?
				};

			let op = Ops::Swap(account.clone(), token_from, token_to, amount, nonce,
				new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to);

			PendingReqMap::<T>::insert(&_req_id, op);
			ReqIndex::put(_req_id);

			BalanceMap::<T>::insert((&account, token_from), _new_balance_from);
			BalanceMap::<T>::insert((&account, token_to), _new_balance_to);
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

			let _req_id = req_id_get::<T>()?;
			let _new_nonce = nonce_check::<T>(&account, nonce)?;
			let _pool_index = pool_index::<T>(&token_from, &token_to)?;

			// for user account
			let _new_balance_from = balance_sub::<T>(&account, token_from, amount_from)?;
			let _new_balance_to = balance_sub::<T>(&account, token_to, amount_to)?;
			let _new_share = share_add::<T>(&account, &_pool_index, amount_from.checked_add(amount_to).ok_or(Error::<T>::ShareOverflow)?)?;

			// for pool
			let (new_pool_balance_from, new_pool_balance_to) =
				if token_from > token_to {
					let (pool_balance_to, pool_balance_from) = PoolMap::get(_pool_index);
					let new_pool_balance_from = pool_balance_from.checked_add(amount_from).ok_or(Error::<T>::PoolBalanceOverflow)?;
					let new_pool_balance_to = pool_balance_to.checked_add(amount_to).ok_or(Error::<T>::PoolBalanceOverflow)?;			
					PoolMap::insert(_pool_index, (new_pool_balance_to, new_pool_balance_from));
					(new_pool_balance_to, new_pool_balance_from)
				} else if token_from < token_to {
					let (pool_balance_from, pool_balance_to) = PoolMap::get(_pool_index);
					let new_pool_balance_from = pool_balance_from.checked_add(amount_from).ok_or(Error::<T>::PoolBalanceOverflow)?;
					let new_pool_balance_to = pool_balance_to.checked_add(amount_to).ok_or(Error::<T>::PoolBalanceOverflow)?;			
					PoolMap::insert(_pool_index, (new_pool_balance_from, new_pool_balance_to));
					(new_pool_balance_from, new_pool_balance_to)
				} else {
					Err(Error::<T>::InvalidPool)?
				};

			let op = Ops::PoolSupply(
				account.clone(), token_from, token_to, amount_from, amount_to, nonce,
				new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to, _new_share);

			PendingReqMap::<T>::insert(&_req_id, op);
			ReqIndex::put(_req_id);

			BalanceMap::<T>::insert((&account, token_from), _new_balance_from);
			BalanceMap::<T>::insert((&account, token_to), _new_balance_to);
			ShareMap::<T>::insert((&account, _pool_index), _new_share);
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
			let _req_id = req_id_get::<T>()?;
			let _new_nonce = nonce_check::<T>(&account, nonce)?;
			let _pool_index = pool_index::<T>(&token_from, &token_to)?;

			// for user account
			let _new_balance_from = balance_add::<T>(&account, token_from, amount_from)?;
			let _new_balance_to = balance_add::<T>(&account, token_to, amount_to)?;
			let _new_share = share_sub::<T>(&account, &_pool_index, amount_from.checked_add(amount_to).ok_or(Error::<T>::ShareNotEnough)?)?;

			// for pool
			let (new_pool_balance_from, new_pool_balance_to) =
				if token_from > token_to {
					let (pool_balance_to, pool_balance_from) = PoolMap::get(_pool_index);
					let new_pool_balance_from = pool_balance_from.checked_sub(amount_from).ok_or(Error::<T>::PoolBalanceNotEnough)?;
					let new_pool_balance_to = pool_balance_to.checked_sub(amount_to).ok_or(Error::<T>::PoolBalanceNotEnough)?;			
					PoolMap::insert(_pool_index, (new_pool_balance_to, new_pool_balance_from));
					(new_pool_balance_to, new_pool_balance_from)
				} else if token_from < token_to {
					let (pool_balance_from, pool_balance_to) = PoolMap::get(_pool_index);
					let new_pool_balance_from = pool_balance_from.checked_sub(amount_from).ok_or(Error::<T>::PoolBalanceNotEnough)?;
					let new_pool_balance_to = pool_balance_to.checked_sub(amount_to).ok_or(Error::<T>::PoolBalanceNotEnough)?;			
					PoolMap::insert(_pool_index, (new_pool_balance_from, new_pool_balance_to));
					(new_pool_balance_from, new_pool_balance_to)
				} else {
					Err(Error::<T>::InvalidPool)?
				};

			let op = Ops::PoolRetrieve(
				account.clone(), token_from, token_to, amount_from, amount_to, nonce,
				new_pool_balance_from, new_pool_balance_to, _new_balance_from, _new_balance_to, _new_share);

			PendingReqMap::<T>::insert(&_req_id, op);
			ReqIndex::put(_req_id);

			BalanceMap::<T>::insert((&account, token_from), _new_balance_from);
			BalanceMap::<T>::insert((&account, token_to), _new_balance_to);
			ShareMap::<T>::insert((&account, _pool_index), _new_share);
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
