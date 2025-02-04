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

use crate::{error::Error, state::RelayState, LightValidationState};
use itc_parentchain_test::parentchain_header_builder::ParentchainHeaderBuilder;
use itp_sgx_io::StaticSealedIO;
use itp_types::Block;

/// A seal that returns a mock validator.
#[derive(Clone)]
pub struct LightValidationStateSealMock;

impl StaticSealedIO for LightValidationStateSealMock {
	type Error = Error;
	type Unsealed = LightValidationState<Block>;

	fn unseal_from_static_file() -> Result<Self::Unsealed, Self::Error> {
		Ok(LightValidationState::new(RelayState::new(
			ParentchainHeaderBuilder::default().build(),
			Default::default(),
		)))
	}

	fn seal_to_static_file(_unsealed: &Self::Unsealed) -> Result<(), Self::Error> {
		Ok(())
	}
}
