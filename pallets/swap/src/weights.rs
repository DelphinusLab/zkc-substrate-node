
//! Autogenerated weights for pallet_swap
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-12-27, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
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
	fn charge_benchmark() -> Weight {
		(53_440_000 as Weight)
	}
	fn set_key_benchmark() -> Weight {
		(6_090_022_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn add_pool_benchmark() -> Weight {
		(405_639_214_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn deposit_benchmark() -> Weight {
		(406_562_710_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn withdraw_benchmark() -> Weight {
		(403_537_684_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn swap_benchmark() -> Weight {
		(394_008_459_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn pool_supply_benchmark() -> Weight {
		(410_126_543_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn pool_retrieve_benchmark() -> Weight {
		(408_518_911_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn ack() -> Weight {
		(149_490_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(20 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
}