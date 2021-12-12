#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use sp_std::{boxed::*, vec, vec::*};

benchmarks! {
    add_pool_benchmark {
        let caller: T::AccountId = <T as Config>::ADMIN1::get();
        NonceMap::<T>::insert(&caller, U256::from(1));
    }: add_pool(RawOrigin::Signed(caller), 10u32.into(), 20u32.into(), 1u32.into())
}
