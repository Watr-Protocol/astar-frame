// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_xc_asset_config
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `devserver-01`, CPU: `Intel(R) Xeon(R) E-2236 CPU @ 3.40GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("shibuya-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/astar-collator
// benchmark
// pallet
// --chain=shibuya-dev
// --steps=50
// --repeat=20
// --pallet=pallet_xc_asset_config
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./benchmark-results/xc_asset_config_weights.rs
// --template=./scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_xc_asset_config.
pub trait WeightInfo {
	fn register_asset_location() -> Weight;
	fn set_asset_units_per_second() -> Weight;
	fn change_existing_asset_location() -> Weight;
	fn remove_payment_asset() -> Weight;
	fn remove_asset() -> Weight;
}

/// Weights for pallet_xc_asset_config using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Proof Skipped: XcAssetConfig AssetIdToLocation (max_values: None, max_size: None, mode: Measured)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Proof Skipped: EVM AccountCodes (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	fn register_asset_location() -> Weight {
		// Minimum execution time: 15_540 nanoseconds.
		Weight::from_parts(16_114_000, 0)
			.saturating_add(Weight::from_parts(0, 2493))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: XcAssetConfig AssetLocationToId (r:1 w:0)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	fn set_asset_units_per_second() -> Weight {
		// Minimum execution time: 15_297 nanoseconds.
		Weight::from_parts(15_551_000, 0)
			.saturating_add(Weight::from_parts(0, 2661))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Proof Skipped: XcAssetConfig AssetIdToLocation (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:1 w:2)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:2)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	fn change_existing_asset_location() -> Weight {
		// Minimum execution time: 22_357 nanoseconds.
		Weight::from_parts(22_572_000, 0)
			.saturating_add(Weight::from_parts(0, 5373))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	fn remove_payment_asset() -> Weight {
		// Minimum execution time: 9_707 nanoseconds.
		Weight::from_parts(10_005_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Proof Skipped: XcAssetConfig AssetIdToLocation (max_values: None, max_size: None, mode: Measured)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Proof Skipped: EVM AccountCodes (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	fn remove_asset() -> Weight {
		// Minimum execution time: 18_645 nanoseconds.
		Weight::from_parts(18_878_000, 0)
			.saturating_add(Weight::from_parts(0, 2987))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Proof Skipped: XcAssetConfig AssetIdToLocation (max_values: None, max_size: None, mode: Measured)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Proof Skipped: EVM AccountCodes (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	fn register_asset_location() -> Weight {
		// Minimum execution time: 15_540 nanoseconds.
		Weight::from_parts(16_114_000, 0)
			.saturating_add(Weight::from_parts(0, 2493))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: XcAssetConfig AssetLocationToId (r:1 w:0)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	fn set_asset_units_per_second() -> Weight {
		// Minimum execution time: 15_297 nanoseconds.
		Weight::from_parts(15_551_000, 0)
			.saturating_add(Weight::from_parts(0, 2661))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Proof Skipped: XcAssetConfig AssetIdToLocation (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:1 w:2)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:2)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	fn change_existing_asset_location() -> Weight {
		// Minimum execution time: 22_357 nanoseconds.
		Weight::from_parts(22_572_000, 0)
			.saturating_add(Weight::from_parts(0, 5373))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	fn remove_payment_asset() -> Weight {
		// Minimum execution time: 9_707 nanoseconds.
		Weight::from_parts(10_005_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Proof Skipped: XcAssetConfig AssetIdToLocation (max_values: None, max_size: None, mode: Measured)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Proof Skipped: EVM AccountCodes (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	// Proof Skipped: XcAssetConfig AssetLocationToId (max_values: None, max_size: None, mode: Measured)
	fn remove_asset() -> Weight {
		// Minimum execution time: 18_645 nanoseconds.
		Weight::from_parts(18_878_000, 0)
			.saturating_add(Weight::from_parts(0, 2987))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
