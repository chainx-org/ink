// Copyright 2019 Chainpool.

use super::{
    chainx_types::{
        Memo,
        Token,
    },
    EnvTypes,
};
use indices::address::Address;
use old_scale::{
    Codec,
    Decode,
    Encode,
};
use sr_primitives::traits::Member;

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Clone, PartialEq, Eq))]
pub enum XAssets<T: EnvTypes, AccountIndex>
where
    T::AccountId: Member + Codec,
    AccountIndex: Member + Codec,
{
    #[allow(non_camel_case_types)]
    #[codec(index = "3")]
    transfer(Address<T::AccountId, AccountIndex>, Token, T::Balance, Memo),
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Clone, PartialEq, Eq))]
pub enum XContracts<T: EnvTypes> {
    #[allow(non_camel_case_types)]
    #[codec(index = "8")]
    convert_to_asset(T::AccountId, #[codec(compact)] T::Balance),
}
