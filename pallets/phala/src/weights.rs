//! Autogenerated weights for pallet_phala
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2021-01-13, STEPS: [10, ], REPEAT: 2, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet-phala.
pub trait ModuleWeightInfo {
	fn push_command() -> Weight;
	fn set_stash() -> Weight;
	fn set_payout_prefs() -> Weight;
	fn register_worker() -> Weight;
	fn force_register_worker() -> Weight;
	fn force_set_contract_key() -> Weight;
	fn start_mining_intention() -> Weight;
	fn stop_mining_intention() -> Weight;
	fn transfer_to_tee() -> Weight;
	fn transfer_to_chain() -> Weight;
	fn sync_worker_message() -> Weight;
	fn force_next_round() -> Weight;
	fn force_add_fire() -> Weight;
	fn add_mrenclave() -> Weight;
	fn remove_mrenclave_by_raw_data() -> Weight;
	fn remove_mrenclave_by_index() -> Weight;
	fn force_set_virtual_tasks() -> Weight;
	fn force_reset_fire() -> Weight;
}

/// Weight functions for pallet_phala.
impl<T: frame_system::Trait> ModuleWeightInfo for frame_system::weights::SubstrateWeight<T> {
	fn push_command() -> Weight {
		(54_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_stash() -> Weight {
		(68_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_payout_prefs() -> Weight {
		(52_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn register_worker() -> Weight {
		(858_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn force_register_worker() -> Weight {
		(115_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn force_set_contract_key() -> Weight {
		(9_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn start_mining_intention() -> Weight {
		(107_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn stop_mining_intention() -> Weight {
		(90_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn transfer_to_tee() -> Weight {
		(135_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn transfer_to_chain() -> Weight {
		(364_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn sync_worker_message() -> Weight {
		(345_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn force_next_round() -> Weight {
		(5_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_add_fire() -> Weight {
		(41_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn add_mrenclave() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_mrenclave_by_raw_data() -> Weight {
		(54_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_mrenclave_by_index() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_set_virtual_tasks() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_reset_fire() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl ModuleWeightInfo for () {
	fn push_command() -> Weight {
		(54_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_stash() -> Weight {
		(68_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_payout_prefs() -> Weight {
		(52_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn register_worker() -> Weight {
		(858_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn force_register_worker() -> Weight {
		(115_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn force_set_contract_key() -> Weight {
		(9_000_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn start_mining_intention() -> Weight {
		(107_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn stop_mining_intention() -> Weight {
		(90_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn transfer_to_tee() -> Weight {
		(135_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn transfer_to_chain() -> Weight {
		(364_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn sync_worker_message() -> Weight {
		(345_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn force_next_round() -> Weight {
		(5_000_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_add_fire() -> Weight {
		(41_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn add_mrenclave() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_mrenclave_by_raw_data() -> Weight {
		(54_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_mrenclave_by_index() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_set_virtual_tasks() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_reset_fire() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
