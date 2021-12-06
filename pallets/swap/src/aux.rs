use super::*;

pub fn is_admin<T: Config>(who: &T::AccountId) -> Result<(), Error<T>> {
    if *who == T::ADMIN1::get() || *who == T::ADMIN2::get() {
        return Ok(());
    }

    return Err(Error::<T>::NoAccess);
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
        .valid_on_bn128()
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
        .checked_add_on_bn128(amount)
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
    let (token_index_0, token_index_1, amount_0, amount_1) =
        PoolMap::get(pool_index).ok_or(Error::<T>::PoolNotExists)?;
    let new_amount_0 = if is_add_0 {
        amount_0
            .checked_add_on_bn128(change_0)
            .ok_or(Error::<T>::PoolBalanceOverflow)?
    } else {
        amount_0
            .checked_sub(change_0)
            .ok_or(Error::<T>::PoolBalanceNotEnough)?
    };
    let new_amount_1 = if is_add_1 {
        amount_1
            .checked_add_on_bn128(change_1)
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
pub fn share_add<T: Config>(
    account_index: &AccountIndex,
    pool_index: &PoolIndex,
    amount: Amount,
) -> Result<Amount, Error<T>> {
    let new_amount = ShareMap::get((account_index, pool_index))
        .checked_add_on_bn128(amount)
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

pub fn req_id_get<T: Config>() -> Result<ReqId, Error<T>> {
    let req_id = ReqIndex::get()
        .checked_add_on_bn128(U256::from(1))
        .ok_or(Error::<T>::ReqIdOverflow)?;
    return Ok(req_id);
}

pub trait U256ToByte {
    fn to_le_bytes(&self) -> [u8; 32];
}

impl U256ToByte for U256 {
    fn to_le_bytes(&self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        self.to_little_endian(&mut buf);
        buf
    }
}

pub trait Bn128<T> {
    fn checked_add_on_bn128(&self, rhs: T) -> Option<T>;
    fn valid_on_bn128(&self) -> Option<T>;
}

impl Bn128<U256> for U256 {
    fn checked_add_on_bn128(&self, rhs: U256) -> Option<U256> {
        match self.checked_add(rhs) {
            None => None,
            Some(res) => res.valid_on_bn128(),
        }
    }

    fn valid_on_bn128(&self) -> Option<U256> {
        let maximum = U256::from_dec_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495617",
        )
        .ok()?;
        match *self >= maximum {
            true => None,
            false => Some(*self),
        }
    }
}

pub fn u256_to_bigint(x: &U256) -> BigInt {
    BigInt::from_bytes_le(Sign::Plus, &x.to_le_bytes())
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
