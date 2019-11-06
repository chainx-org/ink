// Copyright 2019 Chainpool
//
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

/// The XRML fundamental types.
///
/// The differences between DefaultSrmlTypes and DefaultXrmlTypes are:
/// 1. Balance uses u64 in DefaultXrmlTypes instead of u128.
/// 2. Call contains the dispatchable Calls in ChainX, currently supporting PCX transfer only.
///
/// Otherwise DefaultSrmlTypes and DefaultXrmlTypes are actually the same.
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
pub enum DefaultXrmlTypes {}

/// The default ChainX call type.
#[derive(Encode, Decode)]
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
pub enum Call {
    #[codec(index = "8")]
    XAssets(calls::XAssets<DefaultXrmlTypes, AccountIndex>),
    #[codec(index = "20")]
    XContracts(calls::XContracts<DefaultXrmlTypes>),
}

impl From<calls::XAssets<DefaultXrmlTypes, AccountIndex>> for Call {
    fn from(xassets_call: calls::XAssets<DefaultXrmlTypes, AccountIndex>) -> Call {
        Call::XAssets(xassets_call)
    }
}

impl From<calls::XContracts<DefaultXrmlTypes>> for Call {
    fn from(xcontracts_call: calls::XContracts<DefaultXrmlTypes>) -> Call {
        Call::XContracts(xcontracts_call)
    }
}

impl EnvTypes for DefaultXrmlTypes {
    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Moment = Moment;
    type BlockNumber = BlockNumber;
    type Call = Call;
}

/// The default XRML address type.
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

/// The default XRML balance type.
pub type Balance = u64;

/// The default XRML account index type.
pub type AccountIndex = u32;

/// The default XRML hash type.
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

/// The default XRML moment type.
pub type Moment = u64;

/// The default XRML blocknumber type.
pub type BlockNumber = u64;

impl Flush for AccountId {
    fn flush(&mut self) {}
}

impl Flush for Hash {
    fn flush(&mut self) {}
}
