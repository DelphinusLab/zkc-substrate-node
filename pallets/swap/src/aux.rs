use super::*;

pub fn is_admin<T: Config>(who: &T::AccountId) -> Result<(), Error<T>> {
    let _ = T::AckAdmins::get().iter().position(|x| x== who).ok_or(Error::<T>::NoAccess)?;
    return Ok(());
}

pub fn nonce_check<T: Config>(account: &T::AccountId, nonce: NonceId) -> Result<NonceId, Error<T>> {
    if nonce != NonceMap::<T>::get(account) {
        return Err(Error::<T>::NonceInconsistent);
    }

    let new_nonce = nonce.checked_add(1u64).ok_or(Error::<T>::NonceOverflow)?;
    return Ok(new_nonce);
}

pub fn l1account_check<T: Config>(l1account: L1Account) -> Result<L1Account, Error<T>> {
    l1account
        .valid_on_circuit()
        .ok_or(Error::<T>::L1AccountOverflow)?;
    return Ok(l1account);
}

/* ---- Account Index ---- */
pub fn get_account_index<T: Config>(account: &T::AccountId) -> Result<AccountIndex, Error<T>> {
    let account_index = AccountIndexMap::<T>::get(&account).ok_or(Error::<T>::AccountNotExists)?;
    return Ok(account_index);
}

pub fn create_account_index<T: Config>(account: &T::AccountId) -> Result<AccountIndex, Error<T>> {
    if get_account_index::<T>(account).is_ok() {
        return Err(Error::<T>::AccountExists);
    }

    let index = AccountIndexCount::get();
    if index >= MAX_ACCOUNT_COUNT {
        return Err(Error::<T>::AccountIndexOverflow);
    }
    AccountIndexCount::set(index + 1);
    AccountIndexMap::<T>::insert(account, index);
    return Ok(index);
}

/* ---- Pool Index ---- */
pub fn get_pool_index<T: Config>(
    token_src_index: &TokenIndex,
    token_dst_index: &TokenIndex,
) -> Result<TokenIndex, Error<T>> {
    let pool_index =
        PoolIndexMap::get((token_src_index, token_dst_index)).ok_or(Error::<T>::PoolNotExists)?;
    return Ok(pool_index);
}

pub fn create_pool_index<T: Config>(
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
            U256::from(0)
        ),
    );
    return Ok(index);
}

/* ---- Balance ---- */

pub fn balance_add<T: Config>(
    account_index: &AccountIndex,
    token_index: &TokenIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = BalanceMap::get((&account_index, token_index))
        .checked_add_on_circuit(amount)
        .ok_or(Error::<T>::BalanceOverflow)?;
    return Ok(new_amount);
}

pub fn balance_sub<T: Config>(
    account_index: &AccountIndex,
    token_index: &TokenIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = BalanceMap::get((&account_index, token_index))
        .checked_sub(amount)
        .ok_or(Error::<T>::BalanceNotEnough)?;
    return Ok(new_amount);
}

pub fn balance_set(account_index: &AccountIndex, token_index: &TokenIndex, amount: Amount) -> () {
    BalanceMap::insert((&account_index, token_index), amount);
}

/* ---- Pool ---- */
pub fn pool_change<T: Config>(
    pool_index: &PoolIndex,
    is_add_0: bool,
    change_0: Amount,
    is_add_1: bool,
    change_1: Amount,
) -> Result<(Amount, Amount), Error<T>> {
    let (token_index_0, token_index_1, amount_0, amount_1, total_share) =
        PoolMap::get(pool_index).ok_or(Error::<T>::PoolNotExists)?;
    let new_amount_0 = if is_add_0 {
        amount_0
            .checked_add_on_circuit(change_0)
            .ok_or(Error::<T>::PoolBalanceOverflow)?
    } else {
        amount_0
            .checked_sub(change_0)
            .ok_or(Error::<T>::PoolBalanceNotEnough)?
    };
    let new_amount_1 = if is_add_1 {
        amount_1
            .checked_add_on_circuit(change_1)
            .ok_or(Error::<T>::PoolBalanceOverflow)?
    } else {
        amount_1
            .checked_sub(change_1)
            .ok_or(Error::<T>::PoolBalanceNotEnough)?
    };
    PoolMap::insert(
        pool_index,
        (token_index_0, token_index_1, new_amount_0, new_amount_1, total_share)
    );
    return Ok((new_amount_0, new_amount_1));
}

pub fn pool_change_with_share<T: Config>(
    pool_index: &PoolIndex,
    is_add_0: bool,
    change_0: Amount,
    is_add_1: bool,
    change_1: Amount,
    amount: Amount
) -> Result<(Amount, Amount, Amount), Error<T>> {
    let (token_index_0, token_index_1, amount_0, amount_1, total_share) =
        PoolMap::get(pool_index).ok_or(Error::<T>::PoolNotExists)?;
    let new_amount_0 = if is_add_0 {
        amount_0
            .checked_add_on_circuit(change_0)
            .ok_or(Error::<T>::PoolBalanceOverflow)?
    } else {
        amount_0
            .checked_sub(change_0)
            .ok_or(Error::<T>::PoolBalanceNotEnough)?
    };
    let new_amount_1 = if is_add_1 {
        amount_1
            .checked_add_on_circuit(change_1)
            .ok_or(Error::<T>::PoolBalanceOverflow)?
    } else {
        amount_1
            .checked_sub(change_1)
            .ok_or(Error::<T>::PoolBalanceNotEnough)?
    };
    let total_share_new = if is_add_0 {
        total_share
            .checked_add_on_circuit(amount)
            .ok_or(Error::<T>::ShareOverflow)?
    } else {
        total_share
            .checked_sub(amount)
            .ok_or(Error::<T>::ShareNotEnough)?
    };
    PoolMap::insert(
        pool_index,
        (token_index_0, token_index_1, new_amount_0, new_amount_1, total_share_new)
    );
    return Ok((new_amount_0, new_amount_1, total_share_new));
}

/* ---- Share ---- */
pub fn share_add<T: Config>(
    account_index: &AccountIndex,
    pool_index: &PoolIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = ShareMap::get((account_index, pool_index))
        .checked_add_on_circuit(amount)
        .ok_or(Error::<T>::ShareOverflow)?;
    return Ok(new_amount);
}

pub fn share_sub<T: Config>(
    account_index: &AccountIndex,
    pool_index: &PoolIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = ShareMap::get((&account_index, pool_index))
        .checked_sub(amount)
        .ok_or(Error::<T>::ShareNotEnough)?;
    return Ok(new_amount);
}

pub fn get_share_change<T: Config>(
    pool_index: &PoolIndex,
    amount: Amount,
    is_supply: bool
) -> Result<Amount, Error<T>>{
    let (_, _, amount0, _, total_share) = PoolMap::get(pool_index).ok_or(Error::<T>::PoolNotExists)?;

    valid_pool_amount(amount).ok_or(Error::<T>::InvalidAmount)?;

    let share_change = if is_supply {
        if total_share != U256::from(0) {
            let dividend = (amount * total_share).valid_on_circuit().ok_or(Error::<T>::InternalMulOverflow)?;
            let divisor = amount0;
            dividend / divisor
        } else {
            let initial_amount = (amount * U256::exp10(ORDER_OF_MAGNITUDE)).valid_on_circuit().ok_or(Error::<T>::InternalMulOverflow)?;
            initial_amount
        }
    } else {
        let dividend = (amount * total_share).valid_on_circuit().ok_or(Error::<T>::InternalMulOverflow)?;
        let divisor = amount0;
        let share = dividend / divisor;
        let rem = dividend % divisor;

        if rem != U256::from(0) {
            share + 1
        } else {
            share
        }
    };
    return Ok(share_change);
}

pub fn calculate_swap_result_amount<T: Config>(
    amount_input: Amount,
    amount_output: Amount,
    amount: Amount
) -> Result<Amount, Error<T>> {
    valid_pool_amount(amount_input).ok_or(Error::<T>::InvalidAmount)?;
    valid_pool_amount(amount_output).ok_or(Error::<T>::InvalidAmount)?;
    valid_pool_amount(amount).ok_or(Error::<T>::InvalidAmount)?;

    // swap rate is almost equal to 0.3%(1021/1024 for convenience in circom)
    let dividend: Amount = (amount_output * amount * U256::from(1021)).valid_on_circuit().ok_or(Error::<T>::InternalMulOverflow)?;
    let divisor: Amount = ((amount_input + amount) * U256::from(1024)).valid_on_circuit().ok_or(Error::<T>::InternalMulOverflow)?;
    let result_amount = dividend.checked_div(divisor).unwrap();
    return Ok(result_amount);
}

pub fn valid_pool_amount(
    amount: Amount
) -> Option<U256> {
    let maximum = U256::from(1u64) << 99;
    match amount >= maximum {
        true => None,
        false => Some(amount),
    }
}

pub fn valid_input_y_amount(
    input_y_mul_liq0: Amount,
    input_x_mul_liq1: Amount,
    is_supply: bool
) -> Option<U256> {
    let validation: bool;
    if is_supply {
        validation = input_y_mul_liq0 >= input_x_mul_liq1;
    } else {
        validation = input_y_mul_liq0 <= input_x_mul_liq1;
    }
    match validation {
        true => Some(input_y_mul_liq0),
        false => None,
    }
}

/* --- NFT --- */

trait NFTData<T: Config> {
    fn checked_empty(&self) -> Result<(), Error<T>>;
    fn checked_owner(&self, account_index: &AccountIndex) -> Result<(), Error<T>>;
}

impl<T:Config> NFTData<T> for (AccountIndex, Amount, Option<AccountIndex>) {
    fn checked_empty(&self) -> Result<(), Error<T>> {
        if self.0 != 0u32 {
            return Err(Error::<T>::InvalidNFTIndex);
        } else {
            return Ok(());
        }
    }

    fn checked_owner(&self, account_index: &AccountIndex) -> Result<(), Error<T>> {
        if self.0 != *account_index {
            return Err(Error::<T>::IsNotOwner);
        } else {
            return Ok(());
        }
    }
}

pub fn nft_add<T: Config>(
    account_index: &AccountIndex,
    nft_id: &NFTId,
) -> Result<(), Error<T>> {
    let nft = NFTMap::get(&nft_id);
    nft.checked_empty()?;
    let bidder: Option<AccountIndex> = None;
    NFTMap::insert(nft_id, (account_index, U256::from(0), bidder));
    return Ok(());
}

pub fn nft_withdraw<T: Config>(
    account_index: &AccountIndex,
    nft_id: &NFTId,
) -> Result<(), Error<T>> {
    let nft = NFTMap::get(&nft_id);
    nft.checked_owner(account_index)?;
    if nft.2 != None {
        let new_balance_amount = balance_add::<T>(&nft.2.unwrap(), &NFT_TOKEN_INDEX, nft.1)?;
        balance_set(&nft.2.unwrap(), &NFT_TOKEN_INDEX, new_balance_amount);
    }
    let bidder: Option<AccountIndex> = None;
    NFTMap::insert(nft_id, (0, U256::from(0), bidder));
    return Ok(());
}

pub fn nft_transfer<T: Config>(
    from_index: &AccountIndex,
    to_index: &AccountIndex,
    nft_id: &NFTId,
) -> Result<(), Error<T>> {
    let nft = NFTMap::get(&nft_id);
    nft.checked_owner(from_index)?;

    NFTMap::insert(nft_id, (to_index, nft.1, nft.2));
    return Ok(());
}

pub fn nft_bid<T: Config>(
    bidder: &AccountIndex,
    amount: Amount,
    nft_id: &NFTId,
) -> Result<(), Error<T>> {
    let nft = NFTMap::get(&nft_id);
    if nft.0 == 0u32 {
        return Err(Error::<T>::InvalidNFTIndex);
    }
    if nft.2 != None {
        let new_balance_amount0 = balance_add::<T>(&nft.2.unwrap(), &NFT_TOKEN_INDEX, nft.1)?;
        let new_balance_amount1 = balance_sub::<T>(bidder, &NFT_TOKEN_INDEX, amount)?;
        balance_set(&nft.2.unwrap(), &NFT_TOKEN_INDEX, new_balance_amount0);
        balance_set(bidder, &NFT_TOKEN_INDEX, new_balance_amount1);
    } else {
        let new_balance_amount1 = balance_sub::<T>(bidder, &NFT_TOKEN_INDEX, amount)?;
        balance_set(bidder, &NFT_TOKEN_INDEX, new_balance_amount1);

    }
    let bidder: Option<&AccountIndex> = Some(bidder);
    NFTMap::insert(nft_id, (nft.0, amount, bidder));
    return Ok(());
}

pub fn nft_finalize<T: Config>(
    account_index: &AccountIndex,
    nft_id: &NFTId,
) -> Result<(), Error<T>> {
    let nft = NFTMap::get(&nft_id);
    if nft.2 == None {
        return Err(Error::<T>::InvalidNFTIndex);
    }
    nft.checked_owner(account_index)?;
    let new_balance_amount = balance_add::<T>(account_index, &NFT_TOKEN_INDEX, nft.1)?;
    balance_set(account_index, &NFT_TOKEN_INDEX, new_balance_amount);
    let bidder: Option<AccountIndex> = None;
    NFTMap::insert(nft_id, (nft.2.unwrap(), U256::from(0u8), bidder));
    return Ok(());
}

pub fn validation_account_index<T: Config>(account_index: AccountIndex) -> Result<(), Error<T>> {
    if account_index >= AccountIndexCount::get() || account_index == 0u32 {
        return Err(Error::<T>::InvalidAccount);
    }
    return Ok(());
}

pub fn validation_nft_index<T: Config>(nft_id: NFTId) -> Result<(), Error<T>> {
    if nft_id >= MAX_NFTINDEX_COUNT || nft_id == 0u32 {
        return Err(Error::<T>::InvalidNFTIndex);
    }
    return Ok(());
}

pub fn req_id_get<T: Config>() -> Result<ReqId, Error<T>> {
    let req_id = ReqIndex::get()
        .checked_add_on_circuit(U256::from(1))
        .ok_or(Error::<T>::ReqIdOverflow)?;
    return Ok(req_id);
}

pub trait U256ToByte {
    fn to_be_bytes(&self) -> [u8; 32];
}

impl U256ToByte for U256 {
    fn to_be_bytes(&self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        self.to_big_endian(&mut buf);
        buf
    }
}

pub trait CircuitRange<T> {
    fn checked_add_on_circuit(&self, rhs: T) -> Option<T>;
    fn valid_on_circuit(&self) -> Option<T>;
}

impl CircuitRange<U256> for U256 {
    fn checked_add_on_circuit(&self, rhs: U256) -> Option<U256> {
        match self.checked_add(rhs) {
            None => None,
            Some(res) => res.valid_on_circuit(),
        }
    }

    fn valid_on_circuit(&self) -> Option<U256> {
        let maximum = U256::from(1u64) << 250;
        match *self >= maximum {
            true => None,
            false => Some(*self),
        }
    }
}

pub fn u256_to_bigint(x: &U256) -> BigInt {
    BigInt::from_bytes_be(Sign::Plus, &x.to_be_bytes())
}

pub fn u256_from_bigint(x: &BigInt) -> U256 {
    let (_, buf) = x.to_bytes_le();
    U256::from_little_endian(&buf)
}

fn _check_sign<T: Config>(data: &[u8], sign: Signature, key: PublicKey) -> Result<(), Error<T>> {
    let s = delphinus_crypto::Sign::<BabyJubjubField> {
        r: BabyJubjubPoint {
            x: BabyJubjubField::new(&u256_to_bigint(&sign.0)),
            y: BabyJubjubField::new(&u256_to_bigint(&sign.1)),
        },
        s: BabyJubjubField::new(&u256_to_bigint(&sign.2)),
    };
    let k = BabyJubjubPoint {
        x: BabyJubjubField::new(&u256_to_bigint(&key.0)),
        y: BabyJubjubField::new(&u256_to_bigint(&key.1)),
    };

    if BabyJubjub::verify(data, s, k) {
        Ok(())
    } else {
        Err(Error::<T>::InvalidSignature)
    }
}

pub fn check_sign<T: Config>(
    account_index: AccountIndex,
    command: &[u8],
    sign: &[u8],
) -> Result<Signature, Error<T>> {
    let key = KeyMap::get(account_index).ok_or(Error::<T>::InvalidAccount)?;
    let _r = BabyJubjubPoint::decode(&sign[..32]).map_err(|_| Error::<T>::InvalidSignature)?;
    let _s = BabyJubjubField::decode(&sign[32..]);
    let rx = u256_from_bigint(&_r.x.v);
    let ry = u256_from_bigint(&_r.y.v);
    let s = u256_from_bigint(&_s.v);
    let sign = (rx, ry, s);

    _check_sign::<T>(command, sign, key)?;
    Ok(sign)
}
