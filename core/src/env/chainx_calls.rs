// Copyright 2019 Chainpool.

use super::{
    chainx_types::{
        Memo,
        Token,
    },
    EnvTypes,
};
use core::convert::TryInto;
use old_scale::{
    Decode,
    Encode,
    Input,
    Output,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Address<T: EnvTypes, AccountIndex> {
    Id(T::AccountId),
    Index(AccountIndex),
}

fn need_more_than<T: PartialOrd>(a: T, b: T) -> Option<T> {
    if a < b {
        Some(b)
    } else {
        None
    }
}

/// Decode implementation copied over from Substrate `Address` that can be found [here](substrate-address).
///
/// # Note
/// This implementation MUST be kept in sync with substrate, tests below will ensure that.
///
/// [substrate-address]: https://github.com/paritytech/substrate/blob/ec62d24c602912f07bbc416711376d9b8e5782c5/srml/indices/src/address.rs#L61
impl<T, AccountIndex> Decode for Address<T, AccountIndex>
where
    T: EnvTypes,
    AccountIndex: Decode + From<u32> + PartialOrd + Copy + Clone,
{
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        Some(match input.read_byte()? {
            x @ 0x00..=0xef => Address::Index(AccountIndex::from(x as u32)),
            0xfc => {
                Address::Index(AccountIndex::from(need_more_than(
                    0xef,
                    u16::decode(input)? as u32,
                )?))
            }
            0xfd => {
                Address::Index(AccountIndex::from(need_more_than(
                    0xffff,
                    u32::decode(input)?,
                )?))
            }
            0xfe => {
                Address::Index(need_more_than(
                    AccountIndex::from(0xffffffffu32),
                    Decode::decode(input)?,
                )?)
            }
            0xff => Address::Id(Decode::decode(input)?),
            _ => return None,
        })
    }
}

/// Encode implementation copied over from Substrate `Address` that can be found [here](substrate-address).
///
/// # Note
/// This implementation MUST be kept in sync with substrate, tests below will ensure that.
///
/// [substrate-address]: https://github.com/paritytech/substrate/blob/ec62d24c602912f07bbc416711376d9b8e5782c5/srml/indices/src/address.rs#L83
impl<T, AccountIndex> Encode for Address<T, AccountIndex>
where
    T: EnvTypes,
    AccountIndex: Encode + TryInto<u32> + Copy + Clone,
{
    fn encode_to<O: Output>(&self, dest: &mut O) {
        match *self {
            Address::Id(ref i) => {
                dest.push_byte(255);
                dest.push(i);
            }
            Address::Index(i) => {
                let maybe_u32: Result<u32, _> = i.try_into();
                if let Ok(x) = maybe_u32 {
                    if x > 0xffff {
                        dest.push_byte(253);
                        dest.push(&x);
                    } else if x >= 0xf0 {
                        dest.push_byte(252);
                        dest.push(&(x as u16));
                    } else {
                        dest.push_byte(x as u8);
                    }
                } else {
                    dest.push_byte(254);
                    dest.push(&i);
                }
            }
        }
    }
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Clone, PartialEq, Eq))]
pub enum XAssets<T: EnvTypes, AccountIndex> {
    #[allow(non_camel_case_types)]
    #[codec(index = "3")]
    transfer(Address<T, AccountIndex>, Token, T::Balance, Memo),
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Clone, PartialEq, Eq))]
pub enum XContracts<T: EnvTypes> {
    #[allow(non_camel_case_types)]
    #[codec(index = "8")]
    convert_to_asset(T::AccountId, #[codec(compact)] T::Balance),
}
