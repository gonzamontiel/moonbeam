// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_precompile_benchmarks`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_precompile_benchmarks
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_precompile_benchmarks`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_precompile_benchmarks::WeightInfo for WeightInfo<T> {
	/// The range of component `x` is `[100, 2000]`.
	fn verify_entry(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 76_307_000 picoseconds.
		Weight::from_parts(76_780_000, 0)
			// Standard Error: 2_406
			.saturating_add(Weight::from_parts(666_207, 0).saturating_mul(x.into()))
	}
	/// Storage: `RelayStorageRoots::RelayStorageRootKeys` (r:1 w:0)
	/// Proof: `RelayStorageRoots::RelayStorageRootKeys` (`max_values`: Some(1), `max_size`: Some(121), added: 616, mode: `MaxEncodedLen`)
	fn latest_relay_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227`
		//  Estimated: `1606`
		// Minimum execution time: 4_557_000 picoseconds.
		Weight::from_parts(4_709_000, 1606)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}
