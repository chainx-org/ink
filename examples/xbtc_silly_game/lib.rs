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

#[ink::contract(version = "0.1.0", env = DefaultXrmlTypes)]
mod xbtc_silly_game {
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
    }
}
