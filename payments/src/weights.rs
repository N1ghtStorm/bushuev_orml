//! Autogenerated weights for virto_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-19, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/virto-parachain
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution
// compiled
// --extrinsic=*
// --pallet=virto-payment
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output
// ./pallets/payment/src/weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for virto_payment.
pub trait WeightInfo {
	fn pay(x: u32, ) -> Weight;
	fn release() -> Weight;
	fn cancel() -> Weight;
	fn resolve_payment() -> Weight;
	fn request_refund() -> Weight;
	fn dispute_refund() -> Weight;
	fn request_payment() -> Weight;
	fn accept_and_pay() -> Weight;
	fn remove_task() -> Weight;
}

/// Weights for virto_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay(_x: u32, ) -> Weight {
		(55_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		(36_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		(48_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn resolve_payment() -> Weight {
		(35_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn request_refund() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn dispute_refund() -> Weight {
		(21_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	fn request_payment() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn accept_and_pay() -> Weight {
		(58_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn remove_task() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn pay(_x: u32, ) -> Weight {
		(55_900_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		(36_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		(48_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	fn resolve_payment() -> Weight {
		(35_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn request_refund() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn dispute_refund() -> Weight {
		(21_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Sudo Key (r:1 w:0)
	fn request_payment() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Payment Payment (r:1 w:1)
	// Storage: Assets Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn accept_and_pay() -> Weight {
		(58_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Payment ScheduledTasks (r:1 w:1)
	fn remove_task() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}