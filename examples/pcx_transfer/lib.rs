#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

mod crypto {
    /// Do a Blake2 256-bit hash and place result in `dest`.
    pub fn blake2_256_into(data: &[u8], dest: &mut [u8; 32]) {
        dest.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], data).as_bytes());
    }

    /// Do a Blake2 256-bit hash and return result.
    pub fn blake2_256(data: &[u8]) -> [u8; 32] {
        let mut r = [0; 32];
        blake2_256_into(data, &mut r);
        r
    }
}

#[ink::contract(version = "0.1.0", env = DefaultXrmlTypes)]
mod pcx_transfer {
    use super::crypto;
    use ink_core::{
        env::{
            chainx_calls,
            chainx_types::{
                AccountIndex,
                Call,
            },
            DefaultXrmlTypes,
        },
        storage,
    };
    use ink_prelude::collections::BTreeMap;
    use scale::{
        Decode,
        Encode,
        KeyedVec,
    };

    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Encode, Decode)]
    #[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
    pub enum AssetType {
        Free,
        ReservedStaking,
        ReservedStakingRevocation,
        ReservedWithdrawal,
        ReservedDexSpot,
        ReservedDexFuture,
        ReservedCurrency,
        ReservedXRC20,
        GasPayment,
    }

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

        /// Dispatches a `transfer` call to the ChainX Assets module.
        #[ink(message)]
        fn pcx_transfer(&mut self, dest: AccountId, value: u64) {
            let dest_addr = chainx_calls::Address::Id(dest);
            let transfer_call = Call::XAssets(chainx_calls::XAssets::<
                DefaultXrmlTypes,
                AccountIndex,
            >::transfer(
                dest_addr,
                b"PCX".to_vec(),
                value,
                b"memo".to_vec(),
            ));
            let _ = self.env().invoke_runtime(&transfer_call);
        }

        /// Returns the account PCX asset balance, read directly from runtime storage
        #[ink(message)]
        fn get_asset_balance(
            &self,
            account: AccountId,
        ) -> Option<Result<BTreeMap<AssetType, u64>, ()>> {
            const BALANCE_OF: &[u8] = b"XAssets AssetBalance";
            let pcx_balance = (account, b"PCX".to_vec());
            let key = crypto::blake2_256(&pcx_balance.to_keyed_vec(BALANCE_OF));
            let result = self
                .env()
                .get_runtime_storage::<BTreeMap<AssetType, u64>>(&key[..]);
            result.map(|x| x.map_err(|_| ()))
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
