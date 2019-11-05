#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

use ink_core::{
    env::{
        chainx_calls,
        AccountIndex,
        DefaultXrmlTypes,
    },
    memory::format,
    storage,
};
use ink_lang::contract;

contract! {
    #![env = DefaultXrmlTypes]

    /// This simple dummy contract has a `bool` value that can
    /// alter between `true` and `false` using the `flip` message.
    /// Users can retrieve its current state using the `get` message.
    struct PcxTransfer {
        /// The current state of our flag.
        value: storage::Value<bool>,
    }

    impl Deploy for PcxTransfer {
        /// Initializes our state to `false` upon deploying our smart contract.
        fn deploy(&mut self) {
            self.value.set(false)
        }
    }

    impl PcxTransfer {
        /// Flips the current state of our smart contract.
        pub(external) fn flip(&mut self) {
            *self.value = !*self.value;
        }

        /// Dispatches a `transfer` call to the Balances srml module
        pub(external) fn pcx_transfer(&mut self, dest: AccountId, value: Balance, memo: Vec<u8>) {
            let dest_addr = chainx_calls::Address::Id(dest);
            env.println(&format!("pcx_transfer dest: {:?}, value: {:?}", dest, value));
            let transfer_call = chainx_calls::XAssets::<DefaultXrmlTypes, AccountIndex>::transfer(dest_addr, b"PCX".to_vec(), value, memo);
            env.dispatch_call(transfer_call);
        }

        /// Returns the current state.
        pub(external) fn get(&self) -> bool {
            env.println(&format!("Storage Value: {:?}", *self.value));
            *self.value
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut contract = PcxTransfer::deploy_mock();
        assert_eq!(contract.get(), false);
        contract.flip();
        assert_eq!(contract.get(), true);
    }
}
