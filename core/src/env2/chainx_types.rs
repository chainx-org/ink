// Copyright 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Types for the default SRML environment.
//!
//! These are simple mirrored types from the default SRML configuration.
//! Their interfaces and functionality might not be complete.
//!
//! Users are required to provide their own type definitions and `EnvTypes`
//! implementations in order to write ink! contracts for other chain configurations.

use old_scale::{
    Decode,
    Encode,
};
#[cfg(feature = "ink-generate-abi")]
use type_metadata::Metadata;

use ink_prelude::vec::Vec;

use crate::env2::EnvTypes;

use super::chainx_calls::{
    XAssets,
    XContracts,
};

/// The fundamental types of the SRML default configuration.
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
#[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
pub enum DefaultXrmlTypes {}

impl EnvTypes for DefaultXrmlTypes {
    type AccountId = super::types::AccountId;
    type Balance = Balance;
    type Hash = super::types::Hash;
    type Moment = Moment;
    type BlockNumber = BlockNumber;
    type Call = Call;
}

/// The default XRML balance type.
pub type Balance = u64;

/// The default SRML moment type.
pub type Moment = u64;

/// The default SRML blocknumber type.
pub type BlockNumber = u64;

/// The default XRML account index type.
pub type AccountIndex = u32;

/// The default XRML xassets token type.
pub type Token = Vec<u8>;

/// The default XRML transaction memo type.
pub type Memo = Vec<u8>;

/// Empty enum for default Call type, so it cannot be constructed.
/// For calling into the runtime, a user defined Call type required.
/// See https://github.com/paritytech/ink-types-node-runtime.
///
/// # Note
///
/// Some traits are only implemented to satisfy the constraints of the test
/// environment, in order to keep the code size small.

/// This call type guarantees to never be constructed.
///
/// This has the effect that users of the default SRML types are
/// not able to call back into the runtime.
/// This operation is generally unsupported because of the currently
/// implied additional overhead.
///
/// # Note
///
/// A user defined `Call` type is required for calling into the runtime.
/// For more info visit: https://github.com/paritytech/ink-types-node-runtime
#[derive(Encode, Decode)]
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
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
