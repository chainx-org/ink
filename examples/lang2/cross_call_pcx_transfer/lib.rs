#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_core::env2::DefaultXrmlTypes;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0", env = DefaultXrmlTypes)]
mod cross_call_pcx_transfer {
    #[ink(storage)]
    struct CrossCallPcxTransfer {
        /// The pcx_transfer smart contract.
        pcx_transfer: storage::Value<pcx_transfer::PcxTransfer>,
    }

    #[ink(event)]
    struct DelegatePcxTransfer {
        from: AccountId,
        to: AccountId,
        value: Balance,
    }

    impl CrossCallPcxTransfer {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        fn new(
            &mut self,
            pcx_transfer: AccountId
        ) {
            use ink_core::env2::call::FromAccountId;
            let pcx_transfer_instance = pcx_transfer::PcxTransfer::from_account_id(pcx_transfer);
            self.pcx_transfer.set(pcx_transfer_instance);
        }
 
        /// Delegates the call.
        #[ink(message)]
        fn delegate_pcx_transfer(&mut self, dest: AccountId, value: Balance) {
            self.pcx_transfer.pcx_transfer(dest, value);
            let contract_address = self.env().address();
            self.env().emit_event(DelegatePcxTransfer {
                from: contract_address,
                to: dest,
                value
            });
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
    }
}
