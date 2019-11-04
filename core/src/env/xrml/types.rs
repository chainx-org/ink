// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of ink!.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ink!.  If not, see <http://www.gnu.org/licenses/>.

use core::{
    array::TryFromSliceError,
    convert::TryFrom,
};

use crate::{
    env::EnvTypes,
    storage::Flush,
};
use scale::{
    Decode,
    Encode,
};
#[cfg(feature = "ink-generate-abi")]
use type_metadata::Metadata;

use super::calls;

/// The SRML fundamental types.
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
pub enum DefaultXrmlTypes {}

/// Empty enum for default Call type, so it cannot be constructed.
/// For calling into the runtime, a user defined Call type required.
/// See https://github.com/paritytech/ink-types-node-runtime.
///
/// # Note
///
/// Some traits are only implemented to satisfy the constraints of the test
/// environment, in order to keep the code size small.
///
/// The default ChainX call type.
#[derive(Encode, Decode)]
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
pub enum Call {
    #[cfg(feature = "old-codec")]
    #[codec(index = "8")]
    XAssets(calls::XAssets<DefaultXrmlTypes, AccountIndex>),
}

#[cfg(feature = "old-codec")]
impl From<calls::XAssets<DefaultXrmlTypes, AccountIndex>> for Call {
    fn from(xassets_call: calls::XAssets<DefaultXrmlTypes, AccountIndex>) -> Call {
        Call::XAssets(xassets_call)
    }
}

/// This implementation is only to satisfy the Decode constraint in the
/// test environment. Since Call cannot be constructed then just return
/// None, but this should never be called.
// #[cfg(feature = "test-env")]
// impl scale::Decode for Call {
// #[cfg(not(feature = "old-codec"))]
// fn decode<I: scale::Input>(_value: &mut I) -> Result<Self, scale::Error> {
// Err("Call cannot be instantiated".into())
// }
// #[cfg(feature = "old-codec")]
// fn decode<I: scale::Input>(_value: &mut I) -> Option<Self> {
// None
// }
// }

impl EnvTypes for DefaultXrmlTypes {
    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Moment = Moment;
    type BlockNumber = BlockNumber;
    type Call = Call;
}

/// The default SRML address type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "ink-generate-abi", derive(Metadata))]
pub struct AccountId([u8; 32]);

impl From<[u8; 32]> for AccountId {
    fn from(address: [u8; 32]) -> AccountId {
        AccountId(address)
    }
}

impl<'a> TryFrom<&'a [u8]> for AccountId {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<AccountId, TryFromSliceError> {
        let address = <[u8; 32]>::try_from(bytes)?;
        Ok(AccountId(address))
    }
}

/// The default SRML balance type.
pub type Balance = u64;

pub type AccountIndex = u32;

/// The default SRML hash type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "ink-generate-abi", derive(Metadata))]
pub struct Hash([u8; 32]);

impl Default for Hash {
    fn default() -> Self {
        Hash(Default::default())
    }
}

impl From<[u8; 32]> for Hash {
    fn from(hash: [u8; 32]) -> Hash {
        Hash(hash)
    }
}

impl<'a> TryFrom<&'a [u8]> for Hash {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<Hash, TryFromSliceError> {
        let hash = <[u8; 32]>::try_from(bytes)?;
        Ok(Hash(hash))
    }
}

/// The default SRML moment type.
pub type Moment = u64;

/// The default SRML blocknumber type.
pub type BlockNumber = u64;

impl Flush for AccountId {
    fn flush(&mut self) {}
}

impl Flush for Hash {
    fn flush(&mut self) {}
}
