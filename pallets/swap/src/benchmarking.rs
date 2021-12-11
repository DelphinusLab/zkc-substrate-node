#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    add_pool_benchmark {
        let a in 1...1000;
        let b in 2...1001;
        let n in 1...1000;
    }: add_pool(RawOrigin::Root, a.into(), b.into(), n.into())

    impl_benchmark_test_suit!(Pallet, crate::tests::new_test_ext(), crate::tests::Test)
}
