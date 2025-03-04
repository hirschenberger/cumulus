
//! Autogenerated weights for `pallet_ranked_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-06, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/debug/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --steps=2
// --repeat=1
// --pallet=pallet_ranked_collective
// --extrinsic=*
// --execution=native
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_ranked_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ranked_collective::WeightInfo for WeightInfo<T> {
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:1)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:0 w:1)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:0 w:1)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `6986`
		// Minimum execution time: 129_000_000 picoseconds.
		Weight::from_parts(129_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6986))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:11 w:11)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:11 w:11)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:11 w:11)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `575 + r * (281 ±0)`
		//  Estimated: `89494`
		// Minimum execution time: 325_000_000 picoseconds.
		Weight::from_parts(1_695_000_000, 0)
			.saturating_add(Weight::from_parts(0, 89494))
			.saturating_add(T::DbWeight::get().reads(34))
			.saturating_add(T::DbWeight::get().writes(34))
	}
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:1)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:0 w:1)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:0 w:1)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277 + r * (17 ±0)`
		//  Estimated: `6986`
		// Minimum execution time: 206_000_000 picoseconds.
		Weight::from_parts(450_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6986))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipCollective Members (r:1 w:1)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:1)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IdToIndex (r:1 w:1)
	/// Proof: FellowshipCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective IndexToId (r:1 w:1)
	/// Proof: FellowshipCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `575 + r * (71 ±0)`
		//  Estimated: `14024`
		// Minimum execution time: 309_000_000 picoseconds.
		Weight::from_parts(449_000_000, 0)
			.saturating_add(Weight::from_parts(0, 14024))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipCollective Members (r:1 w:0)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective Voting (r:1 w:1)
	/// Proof: FellowshipCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `665`
		//  Estimated: `328970`
		// Minimum execution time: 536_000_000 picoseconds.
		Weight::from_parts(536_000_000, 0)
			.saturating_add(Weight::from_parts(0, 328970))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective VotingCleanup (r:1 w:0)
	/// Proof: FellowshipCollective VotingCleanup (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective Voting (r:100 w:100)
	/// Proof: FellowshipCollective Voting (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `308 + n * (52 ±0)`
		//  Estimated: `262934`
		// Minimum execution time: 246_000_000 picoseconds.
		Weight::from_parts(1_572_000_000, 0)
			.saturating_add(Weight::from_parts(0, 262934))
			.saturating_add(T::DbWeight::get().reads(102))
			.saturating_add(T::DbWeight::get().writes(100))
	}
}
