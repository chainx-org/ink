#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0", env = DefaultXrmlTypes)]
mod xbtc_silly_game {
    use ink_core::{
        env::DefaultXrmlTypes,
        storage,
    };
    use scale::{
        Decode,
        Encode,
    };

    #[ink(storage)]
    struct XbtcSillyGame {
        xrc20_contract: storage::Value<xrc20::XRC20>,
    }

    impl XbtcSillyGame {
        #[ink(constructor)]
        fn new(&mut self, xrc20_contract: AccountId) {
            use ink_core::env::call::FromAccountId;
            let xrc20_instance = xrc20::XRC20::from_account_id(xrc20_contract);
            self.xrc20_contract.set(xrc20_instance);
        }

        /// Transfer some value of xbtc token to this contract.
        ///
        /// Prerequsite: caller needs to call approve(current_contract, value) of xrc20 contract
        #[ink(message)]
        fn deposit(&mut self, value: u64) -> bool {
            let caller = self.env().caller();
            let receiver = self.env().account_id();
            self.xrc20_contract.transfer_from(caller, receiver, value)
        }

        /// Transfer some value of xbtc token from the contract to external account.
        ///
        /// env.caller() ==> XbtcSillyGame contract
        #[ink(message)]
        fn reward(&mut self, receiver: AccountId, value: u64) -> bool {
            self.delegate_xrc20_transfer(receiver, value)
        }

        /// Delegate the `transfer` call of xrc20 contract.
        #[ink(message)]
        fn delegate_xrc20_transfer(&mut self, dest: AccountId, value: u64) -> bool {
            self.xrc20_contract.transfer(dest, value)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            // Note that even though we defined our `#[ink(constructor)]`
            // above as `&mut self` functions that return nothing we can call
            // them in test code as if they were normal Rust constructors
            // that take no `self` argument but return `Self`.
            let pcx_transfer = PcxTransfer::default();
            assert_eq!(pcx_transfer.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut pcx_transfer = PcxTransfer::new(false);
            assert_eq!(pcx_transfer.get(), false);
            pcx_transfer.flip();
            assert_eq!(pcx_transfer.get(), true);
        }
    }
}
