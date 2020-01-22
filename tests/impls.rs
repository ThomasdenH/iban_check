//! This module statically checks whether types implement the expected traits.
use core::convert::TryFrom;
use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::str::FromStr;
use iban::{BaseIban, Iban, ParseBaseIbanError, ParseIbanError};
use static_assertions::assert_impl_all;

assert_impl_all!(
    BaseIban: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Display,
    FromStr,
    TryFrom<&'static str>,
    Send,
    Sync
);
assert_impl_all!(
    Iban: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Display,
    FromStr,
    TryFrom<BaseIban>,
    TryFrom<&'static str>,
    Send,
    Sync
);
assert_impl_all!(
    ParseBaseIbanError: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Send,
    Sync,
    Display
);
assert_impl_all!(
    ParseIbanError: Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Send,
    Sync,
    Display
);

#[cfg(feature = "std")]
assert_impl_all!(ParseBaseIbanError: std::error::Error);
#[cfg(feature = "std")]
assert_impl_all!(ParseIbanError: std::error::Error);

#[cfg(feature = "serde")]
mod impls_serde {
    use super::*;
    use serde::{Deserialize, Serialize};
    assert_impl_all!(BaseIban: Serialize, Deserialize<'static>);
    assert_impl_all!(Iban: Serialize, Deserialize<'static>);
}