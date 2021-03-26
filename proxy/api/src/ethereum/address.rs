use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

/// An [EIP-55](https://eips.ethereum.org/EIPS/eip-55) Ethereum address.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Address(String);

#[derive(Clone, Debug, thiserror::Error)]
#[error("invalid EIP-55 address")]
pub struct InvalidEIP55AddressError;

impl TryFrom<String> for Address {
    type Error = InvalidEIP55AddressError;

    fn try_from(address: String) -> Result<Self, Self::Error> {
        if eip55::validate_address(&address) {
            Ok(Self(address))
        } else {
            Err(InvalidEIP55AddressError)
        }
    }
}

impl FromStr for Address {
    type Err = InvalidEIP55AddressError;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        address.to_string().try_into()
    }
}

impl From<Address> for String {
    fn from(address: Address) -> Self {
        address.0
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
