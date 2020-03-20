#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

//! # Example for XBTC-based Dapp on ChainX
//!
//! This implements a demo for developing a DAPP using XBTC of ChainX.
//!
//! ## Warning
//!
//! This contract is an *example*. It is not complete, and no guarantee for any kind of security.
//!
//! ## Overview
//!
//! A typical flow of XBTC-based DAPP is about 3 steps:
//!
//! 1. User deposits some XBTC token into the contract to participate the game, i.e.,
//!    transfer some XBTC token to the DAPP conrtact.
//!
//! 2. Main logic of the DAPP contract.
//!
//! 3. Reward the winners of the game, i.e., transfer some XBTC from the DAPP contract
//!    to some external user account.
//!
//! Each instantiation of this contract has a set of `owners` and a `requirement` of
//! how many of them need to agree on a `Transaction` for it to be able to be executed.
//! Every owner can submit a transaction and when enough of the other owners confirm
//! it will be able to be executed. The following invariant is enforced by the contract:
//!
//! ## Interface
//!
//! ### Deposit XBTC Token to DAPP Contract
//!
//! `deposit` is used for depositing some XBTC token to this DAPP contract to start the game.
//!
//! Before invoking `deposit` method, user must invoke `approve` method of XRC20 contract ahead,
//! otherwise the deposit would fail.
//!
//! Note:
//! The XRC20 contract address can be retrived by the RPC of ChainX node `chainx_contractXRCTokenInfo`.
//!
//! ### Reward user from DAPP Contract
//!
//! `reward` is to transfer some XBTC token from this DAPP contract to some external account.

use ink_lang as ink;

mod crypto {
    use twox_hash;

    /// Do a XX 128-bit hash and place result in `dest`.
    pub fn twox_128_into(data: &[u8], dest: &mut [u8; 16]) {
        use ::core::hash::Hasher;
        let mut h0 = twox_hash::XxHash::with_seed(0);
        let mut h1 = twox_hash::XxHash::with_seed(1);
        h0.write(data);
        h1.write(data);
        let r0 = h0.finish();
        let r1 = h1.finish();
        use byteorder::{
            ByteOrder,
            LittleEndian,
        };
        LittleEndian::write_u64(&mut dest[0..8], r0);
        LittleEndian::write_u64(&mut dest[8..16], r1);
    }

    /// Do a XX 128-bit hash and return result.
    pub fn twox_128(data: &[u8]) -> [u8; 16] {
        let mut r: [u8; 16] = [0; 16];
        twox_128_into(data, &mut r);
        r
    }
}

#[derive(scale::Encode, scale::Decode)]
pub struct H256Wrapper(btc_primitives::H256);

impl From<btc_primitives::H256> for H256Wrapper {
    fn from(h: btc_primitives::H256) -> Self {
        Self(h)
    }
}

#[cfg(feature = "std")]
impl type_metadata::HasTypeId for H256Wrapper {
    fn type_id() -> type_metadata::TypeId {
        type_metadata::TypeIdCustom::new(
            "H256",
            type_metadata::Namespace::from_module_path("bitcoin_primitives")
                .expect("non-empty Rust identifier namespaces cannot fail"),
            Vec::new(),
        )
        .into()
    }
}

#[cfg(feature = "std")]
impl type_metadata::HasTypeDef for H256Wrapper {
    fn type_def() -> type_metadata::TypeDef {
        use ink_prelude::vec;
        type_metadata::TypeDefTupleStruct::new(vec![type_metadata::UnnamedField::of::<
            [u8; 32],
        >()])
        .into()
    }
}

#[ink::contract(version = "0.1.0", env = DefaultXrmlTypes)]
mod xbtc_silly_game {
    use super::{
        crypto,
        H256Wrapper,
    };
    use btc_primitives::H256;
    use ink_core::{
        env::DefaultXrmlTypes,
        storage,
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
        /// Prerequsite: caller needs to firstly call approve(current_contract, value) of XRC20 contract.
        #[ink(message)]
        fn deposit(&mut self, value: u64) -> bool {
            let caller = self.env().caller();
            let receiver = self.env().account_id();
            self.xrc20_contract.transfer_from(caller, receiver, value)
        }

        /// Transfer some value of xbtc token from this DAPP to some external account.
        /// You might do not want to expose this interface publicly in the production case.
        ///
        /// Delegate the `transfer` call of xrc20 contract.
        #[ink(message)]
        fn reward(&mut self, receiver: AccountId, value: u64) -> bool {
            self.xrc20_contract.transfer(receiver, value)
        }

        /// Returns the account balance, read directly from runtime storage
        #[ink(message)]
        fn get_best_index(&self) -> H256Wrapper {
            const BEST_INDEX: &[u8] = b"XBridgeOfBTC BestIndex";
            let key = crypto::twox_128(BEST_INDEX);
            let result = self.env().get_runtime_storage::<H256>(&key[..]);
            result.unwrap().unwrap().into()
        }
    }
}
