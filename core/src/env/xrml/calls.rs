// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of ink!.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ink!.  If not, see <http://www.gnu.org/licenses/>.

use crate::env::EnvTypes;
use core::convert::TryInto;
use scale::{
    Decode,
    Encode,
    Output,
};

#[cfg(feature = "old-codec")]
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
pub enum Address<T: EnvTypes, AccountIndex> {
    Id(T::AccountId),
    Index(AccountIndex),
}

#[cfg(feature = "old-codec")]
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
#[cfg(feature = "old-codec")]
impl<T, AccountIndex> scale::Decode for Address<T, AccountIndex>
where
    T: EnvTypes,
    AccountIndex: scale::Decode + From<u32> + PartialOrd + Copy + Clone,
{
    fn decode<I: scale::Input>(input: &mut I) -> Option<Self> {
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

#[cfg(feature = "old-codec")]
#[derive(Encode, Decode)]
#[cfg_attr(feature = "test-env", derive(Debug, Clone, PartialEq, Eq))]
pub enum XAssets<T: EnvTypes, AccountIndex> {
    #[allow(non_camel_case_types)]
    #[codec(index = "3")]
    transfer(
        Address<T, AccountIndex>,
        xassets::Token,
        T::Balance,
        xassets::Memo,
    ),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        calls,
        AccountIndex,
        Call,
        ChainXRuntimeTypes,
    };

    use scale::{
        Decode,
        Encode,
    };
    use srml_indices::address;

    #[test]
    fn account_index_serialization() {
        let account_index = 0u32;

        let ink_address = Address::Index(account_index.into());
        let srml_address: address::Address<[u8; 32], u32> =
            address::Address::Index(account_index);

        let ink_encoded = ink_address.encode();
        let srml_encoded = srml_address.encode();

        assert_eq!(srml_encoded, ink_encoded);

        let srml_decoded: address::Address<[u8; 32], u32> =
            Decode::decode(&mut ink_encoded.as_slice())
                .expect("Account Index decodes to srml Address");
        let srml_encoded = srml_decoded.encode();
        let ink_decoded: Address<ChainXRuntimeTypes, u32> =
            Decode::decode(&mut srml_encoded.as_slice())
                .expect("Account Index decodes back to ink type");

        assert_eq!(ink_address, ink_decoded);
    }

    #[test]
    fn account_id_serialization() {
        let account_id = [0u8; 32];

        let ink_address = Address::Id(account_id.into());
        let srml_address: address::Address<[u8; 32], u32> =
            address::Address::Id(account_id);

        let ink_encoded = ink_address.encode();
        let srml_encoded = srml_address.encode();

        assert_eq!(srml_encoded, ink_encoded);

        let srml_decoded: address::Address<[u8; 32], u32> =
            Decode::decode(&mut ink_encoded.as_slice())
                .expect("Account Id decodes to srml Address");
        let srml_encoded = srml_decoded.encode();
        let ink_decoded: Address<ChainXRuntimeTypes, u32> =
            Decode::decode(&mut srml_encoded.as_slice())
                .expect("Account Id decodes decodes back to ink type");

        assert_eq!(ink_address, ink_decoded);
    }

    #[test]
    fn call_balance_transfer() {
        let balance = 10_000;
        let account_index = 0;

        let contract_address = calls::Address::Index(account_index);
        let contract_transfer =
            calls::XAssets::<ChainXRuntimeTypes, AccountIndex>::transfer(
                contract_address,
                b"PCX".to_vec(),
                balance,
                b"memo".to_vec(),
            );
        let contract_call = Call::XAssets(contract_transfer);

        let srml_address = address::Address::Index(account_index);

        let srml_transfer = xassets::Call::<chainx_runtime::Runtime>::transfer(
            srml_address,
            b"PCX".to_vec(),
            balance,
            b"memo".to_vec(),
        );

        let srml_call = chainx_runtime::Call::XAssets(srml_transfer);

        let contract_call_encoded = contract_call.encode();
        let srml_call_encoded = srml_call.encode();

        assert_eq!(srml_call_encoded, contract_call_encoded);

        let srml_call_decoded: chainx_runtime::Call =
            Decode::decode(&mut contract_call_encoded.as_slice())
                .expect("Balances transfer call decodes to srml type");
        let srml_call_encoded = srml_call_decoded.encode();
        let contract_call_decoded: Call =
            Decode::decode(&mut srml_call_encoded.as_slice())
                .expect("Balances transfer call decodes back to contract type");
        assert_eq!(contract_call, contract_call_decoded);
    }
}
