use super::*;
use frame_support::decl_error;

decl_error! {
    pub enum Error for Module<T: Config> {
        NoneValue,
        BalanceOverflow,
        BalanceNotEnough,
        NFTNoAuthority,
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
        L1AccountOverflow,
        InvalidTokenPair,
        InvalidTokenIndex,
        InvalidAmount,
        InvalidKey,
        InvalidSignature,
        InvalidAccount,
        IsNotOwner,
        InvalidNFTIndex,
        InvalidAmountRatio
    }
}
