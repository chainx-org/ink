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

#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::{
    self,
    storage,
};
use ink_lang::contract;

contract! {
    #![env = ink_core::env::DefaultXrmlTypes]

    event DelegatePcxTransfer {
        from: AccountId,
        to: AccountId,
        value: Balance,
    }

    /// Delegates the call to pcx transfer.
    struct CrossCallPcxTransfer {
        /// The pcx_transfer smart contract.
        pcx_transfer: storage::Value<pcx_transfer::PcxTransfer>,
    }

    impl Deploy for CrossCallPcxTransfer {
        /// Initializes the value to the initial value.
        fn deploy(
            &mut self,
            pcx_transfer: AccountId
        ) {
            self.pcx_transfer.set(pcx_transfer::PcxTransfer::from_account_id(pcx_transfer));
        }
    }

    impl CrossCallPcxTransfer {
        /// Delegates the call.
        pub(external) fn delegate_pcx_transfer(&mut self, dest: AccountId, value: Balance) {
            self.pcx_transfer.pcx_transfer(dest, value);
            env.emit(DelegatePcxTransfer {
                from: env.address(),
                to: dest,
                value
            });
        }

    }
}
