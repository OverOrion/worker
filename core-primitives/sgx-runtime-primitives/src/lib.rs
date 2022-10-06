pub mod constants;
pub mod types;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the sgx-runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {

	use sp_runtime::generic;
	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = crate::types::Header;
	/// Opaque block type.
	pub type Block = crate::types::Block;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}
