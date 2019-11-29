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
        fn new(&mut self) {
            self.value.set(false);
        }

        #[ink(constructor)]
        fn default(&mut self) {
            self.value.set(false);
        }

        #[ink(message)]
        fn flip(&mut self) {
            self.env().println("from flip ------------------");
            *self.value = !self.get();
        }

        /// Dispatches a `transfer` call to the ChainX Assets module.
        #[ink(message)]
        fn pcx_transfer(&mut self, dest: AccountId, value: u64) {
            let dest_addr = chainx_calls::Address::Id(dest);
            self.env().println("from contract0 ------------------");
            let transfer_call =
                chainx_calls::XAssets::<DefaultXrmlTypes, AccountIndex>::transfer(
                    dest_addr,
                    b"PCX".to_vec(),
                    value,
                    b"memo".to_vec(),
                );
            self.env().println("from contract1 ------------------");
            self.env().invoke_runtime(&transfer_call);
            self.env().println("from contract2 ------------------");
        }

        #[ink(message)]
        fn get(&self) -> bool {
            self.env().println("from get ------------------");
            *self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_core::env2;

        #[test]
        fn default_works() {
            let flipper = PcxTransfer::default();
            assert_eq!(flipper.get(), false);
        }

        #[test]
        fn it_works() {
            // let mut flipper = PcxTransfer::new(false);
            let mut flipper = PcxTransfer::new();
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        #[test]
        fn dispatches_balances_call() {
            use ink_core::env2::AccessEnv;
            let alice = AccountId::from([0x0; 32]);
            // ink_core::env2::test::TestEnv::<DefaultXrmlTypes>::get_property(alice);
            ink_core::env2::test::TestEnv::<DefaultXrmlTypes>::set_caller(alice);

            let mut pt = PcxTransfer::new();

            println!("{:?}", pt.env().caller());

            // assert_eq!(
            // env::test::dispatched_calls::<DefaultXrmlTypes>()
            // .into_iter()
            // .count(),
            // 0
            // );
            pt.pcx_transfer(alice, 10000);
            // assert_eq!(
            // env::test::dispatched_calls::<DefaultXrmlTypes>()
            // .into_iter()
            // .count(),
            // 1
            // );
        }
    }
}
