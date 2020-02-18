use super::chainx_calls::{
    XAssets,
    XContracts,
};
use crate::env::types::{
    AccountId,
    EnvTypes,
    Hash,
};
use ink_prelude::vec::Vec;
use old_scale::{
    Decode,
    Encode,
};

/// The fundamental types of the default configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
pub enum DefaultXrmlTypes {}

impl EnvTypes for DefaultXrmlTypes {
    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Timestamp = Timestamp;
    type BlockNumber = BlockNumber;
    type Call = Call;
}

/// The default balance type.
pub type Balance = u64;

/// The default timestamp type.
pub type Timestamp = u64;

/// The default block number type.
pub type BlockNumber = u64;

/// The default XRML account index type.
pub type AccountIndex = u32;

/// The default XRML xassets token type.
pub type Token = Vec<u8>;

/// The default XRML transaction memo type.
pub type Memo = Vec<u8>;

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Clone, PartialEq, Eq))]
pub enum Call {
    #[codec(index = "8")]
    XAssets(XAssets<DefaultXrmlTypes, AccountIndex>),
    #[codec(index = "20")]
    XContracts(XContracts<DefaultXrmlTypes>),
}

impl From<XAssets<DefaultXrmlTypes, AccountIndex>> for Call {
    fn from(xassets_call: XAssets<DefaultXrmlTypes, AccountIndex>) -> Self {
        Call::XAssets(xassets_call)
    }
}

impl From<XContracts<DefaultXrmlTypes>> for Call {
    fn from(xcontracts_call: XContracts<DefaultXrmlTypes>) -> Self {
        Call::XContracts(xcontracts_call)
    }
}
