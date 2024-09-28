#![allow(non_snake_case)]

use nutype::nutype;

#[nutype(derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    FromStr,
    Serialize,
    Deserialize
))]
pub struct Bytes(u64);

#[nutype(derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    FromStr,
    Serialize,
    Deserialize
))]
pub struct Secs(u64);

#[nutype(derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    FromStr,
    Serialize,
    Deserialize
))]
pub struct Microsecs(u128);

#[nutype(derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    FromStr,
    Serialize,
    Deserialize
))]
pub struct Nanosecs(u128);
