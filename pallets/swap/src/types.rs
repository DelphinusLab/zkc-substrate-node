use super::*;

pub type NonceId = u64;
pub type ReqId = U256;
pub type Amount = U256;
pub type Reverse = u8;

pub type L1Account = U256;
pub type L1TxHash = U256;

pub type AccountIndex = u32;
pub type TokenIndex = u32;
pub type PoolIndex = u32;
pub type NFTId = u32;

pub type SignatureRX = U256;
pub type SignatureRY = U256;
pub type SignatureS = U256;
pub type Signature = (SignatureRX, SignatureRY, SignatureS);

pub type PublicKeyX = U256;
pub type PublicKeyY = U256;
pub type PublicKey = (PublicKeyX, PublicKeyY);

pub type ReserveU32 = u32;
pub type ReserveU256 = U256;

#[derive(Encode, Decode, Clone, PartialEq)]
pub enum Ops {
    SetKey(SignatureRX, SignatureRY, SignatureS, NonceId, AccountIndex, ReserveU32, PublicKeyX, PublicKeyY),
    Deposit(SignatureRX, SignatureRY, SignatureS, NonceId, AccountIndex, TokenIndex, Amount, ReserveU256, AccountIndex),
    Withdraw(
        SignatureRX, SignatureRY, SignatureS, NonceId,
        AccountIndex, TokenIndex, Amount, L1Account
    ),
    Swap(SignatureRX, SignatureRY, SignatureS, NonceId, AccountIndex, PoolIndex, Reverse, Amount),
    PoolSupply(SignatureRX, SignatureRY, SignatureS, NonceId, AccountIndex, PoolIndex, Amount, Amount),
    PoolRetrieve(SignatureRX, SignatureRY, SignatureS, NonceId, AccountIndex, PoolIndex, Amount, Amount),
    AddPool(SignatureRX, SignatureRY, SignatureS, NonceId, TokenIndex, TokenIndex, ReserveU256, ReserveU256, PoolIndex, AccountIndex),
    AddNFT(
        SignatureRX, SignatureRY, SignatureS, NonceId,
        NFTId
    ),
    DepositNFT(
        SignatureRX, SignatureRY, SignatureS, NonceId,
        AccountIndex, NFTId
    ),
    WithdrawNFT(
        SignatureRX, SignatureRY, SignatureS, NonceId,
        AccountIndex, NFTId, L1Account
    ),
    TransferNFT(
        SignatureRX, SignatureRY, SignatureS, NonceId,
        AccountIndex, AccountIndex, NFTId
    ),
}
