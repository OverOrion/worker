/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use codec::{Decode, Encode};
use itp_types::H256;
use std::marker::PhantomData;

pub mod types;

pub struct Stf<Call, Getter, State, Runtime> {
	phantom_data: PhantomData<(Call, Getter, State, Runtime)>,
}

/// Payload to be sent to peers for a state update.
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct StatePayload<StateUpdate: Encode> {
	/// State hash before the `state_update` was applied.
	state_hash_apriori: H256,
	/// State hash after the `state_update` was applied.
	state_hash_aposteriori: H256,
	/// State diff applied to state with hash `state_hash_apriori`
	/// leading to state with hash `state_hash_aposteriori`.
	state_update: StateUpdate,
}

impl<StateUpdate: Encode> StatePayload<StateUpdate> {
	/// Get state hash before the `state_update` was applied.
	pub fn state_hash_apriori(&self) -> H256 {
		self.state_hash_apriori
	}
	/// Get state hash after the `state_update` was applied.
	pub fn state_hash_aposteriori(&self) -> H256 {
		self.state_hash_aposteriori
	}
	/// Reference to the `state_update`.
	pub fn state_update(&self) -> &StateUpdate {
		&self.state_update
	}

	/// Create new `StatePayload` instance.
	pub fn new(apriori: H256, aposteriori: H256, update: StateUpdate) -> Self {
		Self {
			state_hash_apriori: apriori,
			state_hash_aposteriori: aposteriori,
			state_update: update,
		}
	}
}
