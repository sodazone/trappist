
//! Autogenerated weights for `pallet_lockdown_mode`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Hectors-MBP-14.fritz.box`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("trappist-rococo"), DB CACHE: 1024

// Executed Command:
// ./target/release/trappist-collator
// benchmark
// pallet
// --chain
// trappist-rococo
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_lockdown_mode
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn activate_lockdown_mode() -> Weight;
	fn deactivate_lockdown_mode() -> Weight;
}

/// Weight functions for `pallet_lockdown_mode`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: lockdownMode lockdownModeStatus (r:1 w:1)
	// Storage: XcmpQueue QueueSuspended (r:0 w:1)
	fn activate_lockdown_mode() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(19_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: lockdownMode lockdownModeStatus (r:1 w:1)
	// Storage: XcmpQueue QueueSuspended (r:0 w:1)
	fn deactivate_lockdown_mode() -> Weight {
		// Minimum execution time: 20_000 nanoseconds.
		Weight::from_ref_time(21_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
