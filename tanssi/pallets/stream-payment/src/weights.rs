// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_stream_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `pop-os`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_stream_payment
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// tmp/pallet_stream_payment.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_stream_payment.
pub trait WeightInfo {
	fn open_stream() -> Weight;
	fn close_stream() -> Weight;
	fn perform_payment() -> Weight;
	fn request_change_immediate() -> Weight;
	fn request_change_delayed() -> Weight;
	fn accept_requested_change() -> Weight;
	fn cancel_change_request() -> Weight;
	fn immediately_change_deposit() -> Weight;
}

/// Weights for pallet_stream_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `StreamPayment::NextStreamId` (r:1 w:1)
	/// Proof: `StreamPayment::NextStreamId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::Streams` (r:0 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn open_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `107`
		//  Estimated: `3694`
		// Minimum execution time: 49_066_000 picoseconds.
		Weight::from_parts(55_468_000, 3694)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn close_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 113_779_000 picoseconds.
		Weight::from_parts(116_675_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn perform_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 82_466_000 picoseconds.
		Weight::from_parts(85_659_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn request_change_immediate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 98_788_000 picoseconds.
		Weight::from_parts(187_700_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_change_delayed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `3713`
		// Minimum execution time: 11_772_000 picoseconds.
		Weight::from_parts(13_209_000, 3713)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn accept_requested_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `6196`
		// Minimum execution time: 90_231_000 picoseconds.
		Weight::from_parts(138_090_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn cancel_change_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286`
		//  Estimated: `3751`
		// Minimum execution time: 9_140_000 picoseconds.
		Weight::from_parts(13_298_000, 3751)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn immediately_change_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 84_809_000 picoseconds.
		Weight::from_parts(104_948_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `StreamPayment::NextStreamId` (r:1 w:1)
	/// Proof: `StreamPayment::NextStreamId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::Streams` (r:0 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn open_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `107`
		//  Estimated: `3694`
		// Minimum execution time: 49_066_000 picoseconds.
		Weight::from_parts(55_468_000, 3694)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn close_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 113_779_000 picoseconds.
		Weight::from_parts(116_675_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn perform_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 82_466_000 picoseconds.
		Weight::from_parts(85_659_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn request_change_immediate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 98_788_000 picoseconds.
		Weight::from_parts(187_700_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_change_delayed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `3713`
		// Minimum execution time: 11_772_000 picoseconds.
		Weight::from_parts(13_209_000, 3713)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn accept_requested_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `6196`
		// Minimum execution time: 90_231_000 picoseconds.
		Weight::from_parts(138_090_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn cancel_change_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286`
		//  Estimated: `3751`
		// Minimum execution time: 9_140_000 picoseconds.
		Weight::from_parts(13_298_000, 3751)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(229), added: 2704, mode: `MaxEncodedLen`)
	fn immediately_change_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `6196`
		// Minimum execution time: 84_809_000 picoseconds.
		Weight::from_parts(104_948_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}