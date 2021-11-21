use super::*;

pub type NonceId = u64;
pub type ReqId = U256;
pub type Amount = U256;
pub type Direction = u8;

pub type L1Account = U256;
pub type L1TxHash = U256;

pub type AccountIndex = u32;
pub type TokenIndex = u32;
pub type PoolIndex = u32;

pub type Signature = (U256, U256, U256);
pub type PublicKey = (U256, U256);

#[derive(Encode, Decode, Clone, PartialEq)]
pub enum Ops {
    SetKey(AccountIndex, PublicKey),
    /// Input: account, token, amount, nonce
    Deposit(Signature, AccountIndex, TokenIndex, Amount),
    /// Input: account, l1account, token, amount, nonce
    Withdraw(
        Signature,
        AccountIndex,
        TokenIndex,
        Amount,
        L1Account,
        NonceId,
    ),
    /// Input: account, pool, direction, amount, nonce
    Swap(Signature, AccountIndex, PoolIndex, Amount, Direction, NonceId),
    /// Input: account, pool, amount0, amount1, nonce
    PoolSupply(Signature, AccountIndex, PoolIndex, Amount, Amount, NonceId),
    /// Input: account, pool, amount0, amount1, nonce
    PoolRetrieve(Signature, AccountIndex, PoolIndex, Amount, Amount, NonceId),
    /// Input: token_index_pair
    /// Output: pool_index
    AddPool(Signature, TokenIndex, TokenIndex, PoolIndex),
}
