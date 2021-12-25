
//! Autogenerated weights for pallet_swap
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-12-25, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-swap
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_swap
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=pallets/swap/src/weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_swap.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_swap::WeightInfo for WeightInfo<T> {
	fn add_pool_benchmark() -> Weight {
		(438_677_160_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}
