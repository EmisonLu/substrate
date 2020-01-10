// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use codec::{Encode, Decode};
use sp_core::RuntimeDebug;
use crate::{
	Randomness, VRFProof, VRFOutput, VRFIndex,
	AuthorityIndex, SlotNumber, AuthorityId, SassafrasAuthorityWeight
};

/// A Sassafras pre-digest. The validator pre-commit a VRF proof at `vrf_index`, and now reveal it
/// as `vrf_output`.
///
/// This digest is included in every block, generated by Sassafras consensus engine.
#[derive(Clone, RuntimeDebug, Encode, Decode)]
pub struct PreDigest {
	/// Index of ticket VRF proof that has been previously committed.
	pub ticket_vrf_index: VRFIndex,
	/// Reveal of tocket VRF output.
	pub ticket_vrf_output: VRFOutput,
	/// Validator index.
	pub authority_index: AuthorityIndex,
	/// Corresponding slot number.
	pub slot_number: SlotNumber,
	/// Secondary "Post Block VRF" proof.
	pub post_vrf_proof: VRFProof,
	/// Secondary "Post Block VRF" output.
	pub post_vrf_output: VRFOutput,
}

/// Post-digest about next epoch information.
///
/// This digest is generated by runtime, at the beginning of every epoch.
#[derive(Clone, RuntimeDebug, Encode, Decode)]
pub struct NextEpochDescriptor {
	/// The authorities that generate VRF proofs. Note that those keys will only be generating
	/// blocks two epochs later.
	pub authorities: Vec<(AuthorityId, SassafrasAuthorityWeight)>,
	/// Value of randomness to use for slot-assignment. This is expected to use the secondary "Post
	/// VRF".
	pub randomness: Randomness,
}

/// Post-digest about post-block information such as ticket commitments.
///
/// This digest is generated by runtime, optional, and can be included at every block.
#[derive(Clone, RuntimeDebug, Encode, Decode)]
pub struct PostBlockDescriptor {
	/// Commitments of tickets.
	pub commitments: Vec<VRFProof>,
}
