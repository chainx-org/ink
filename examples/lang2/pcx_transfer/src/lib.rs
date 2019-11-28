#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::{
    env2::{
        chainx_calls,
        chainx_types::AccountIndex,
        DefaultXrmlTypes,
    },
    storage,
};
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod pcx_transfer {
    #[ink(storage)]
    struct PcxTransfer {
        value: storage::Value<bool>,
    }

    impl PcxTransfer {
        #[ink(constructor)]
        fn new(&mut self, init_value: bool) {
            self.value.set(init_value);
        }

        #[ink(constructor)]
        fn default(&mut self) {
            self.new(false)
        }

        #[ink(message)]
        fn flip(&mut self) {
            *self.value = !self.get();
        }

        /// Dispatches a `transfer` call to the ChainX Assets module.
        #[ink(message)]
        fn pcx_transfer(&mut self, dest: AccountId, value: u64) {
            let dest_addr = chainx_calls::Address::Id(dest);
            let transfer_call =
                chainx_calls::XAssets::<DefaultXrmlTypes, AccountIndex>::transfer(
                    dest_addr,
                    b"PCX".to_vec(),
                    value,
                    b"memo".to_vec(),
                );
            self.env().invoke_runtime(&transfer_call);
        }

        #[ink(message)]
        fn get(&self) -> bool {
            *self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let flipper = PcxTransfer::default();
            assert_eq!(flipper.get(), false);
        }

        #[test]
        fn it_works() {
            let mut flipper = PcxTransfer::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
