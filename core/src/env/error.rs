// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use derive_more::From;

#[cfg(any(feature = "std", test, doc))]
use crate::env::engine::off_chain::OffChainError;

/// Descriptive error type
#[cfg(feature = "std")]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error(&'static str);

/// Undescriptive error type when compiled for no std
#[cfg(not(feature = "std"))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error;

impl Error {
    #[allow(unused)]
    #[cfg(feature = "std")]
    /// Error description
    ///
    /// This function returns an actual error str when running in `std`
    /// environment, but `""` on `no_std`.
    pub fn what(&self) -> &'static str {
        self.0
    }

    #[cfg(not(feature = "std"))]
    /// Error description
    ///
    /// This function returns an actual error str when running in `std`
    /// environment, but `""` on `no_std`.
    pub fn what(&self) -> &'static str {
        ""
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for Error {
    #[cfg(feature = "std")]
    fn from(s: &'static str) -> Error {
        Error(s)
    }

    #[cfg(not(feature = "std"))]
    fn from(_s: &'static str) -> Error {
        Error
    }
}

/// Errors that can be encountered upon environmental interaction.
#[derive(Debug, From, PartialEq, Eq)]
pub enum EnvError {
    /// Error upon decoding an encoded value.
    #[cfg(not(feature = "old-codec"))]
    Decode(scale::Error),
    #[cfg(feature = "old-codec")]
    Decode(Error),
    /// An error that can only occure in the off-chain environment.
    #[cfg(any(feature = "std", test, doc))]
    OffChain(OffChainError),
    /// The call to another contract has trapped.
    ContractCallTrapped,
    /// A called contract returned a custom error code.
    #[from(ignore)]
    ContractCallFailState(u8),
    /// The instantiation of another contract has trapped.
    ContractInstantiationTrapped,
    /// The instantiated contract returned a custom error code.
    #[from(ignore)]
    ContractInstantiationFailState(u8),
    /// The queried runtime storage entry is missing.
    MissingRuntimeStorageEntry,
    /// The queried contract storage entry is missing.
    MissingContractStorageEntry,
    /// A call to transfer value from the contract failed.
    TransferCallFailed,
}

/// A result of environmental operations.
pub type Result<T> = core::result::Result<T, EnvError>;
