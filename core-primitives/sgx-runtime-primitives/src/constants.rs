use crate::types::{BlockNumber, Moment};

use sp_version::create_runtime_str;
use sp_version::RuntimeVersion;
pub use sp_runtime::{Perbill, Permill};
// #[cfg(feature = "evm")]
// pub use evm::{
// 	AddressMapping, EnsureAddressTruncated, EvmCall, FeeCalculator, FixedGasPrice,
// 	FixedGasWeightMapping, GasWeightMapping, HashedAddressMapping, IntoAddressMapping,
// 	SubstrateBlockHashMapping, GAS_PER_SECOND, MAXIMUM_BLOCK_WEIGHT, WEIGHT_PER_GAS,
// };

// use frame_support::weights::ConstantMultiplier;
// use pallet_transaction_payment::CurrencyAdapter;
// use sp_core::OpaqueMetadata;


// // A few exports that help ease life for downstream crates.
// pub use frame_support::{
// 	construct_runtime, parameter_types,
// 	traits::{KeyOwnerProofSystem, Randomness},
// 	weights::{
// 		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
// 		IdentityFee, Weight,
// 	},
// 	StorageValue,
// };
// pub use pallet_balances::Call as BalancesCall;
// pub use pallet_parentchain::Call as ParentchainCall;
// pub use pallet_timestamp::Call as TimestampCall;
// #[cfg(any(feature = "std", test))]
// pub use sp_runtime::BuildStorage;





pub const ONE_DAY: Moment = 86_400_000;

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("node-template"),
	impl_name: create_runtime_str!("node-template"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
