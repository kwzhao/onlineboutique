use chrono::{Datelike, Utc};
use fake::faker::address::en::*;
use fake::faker::creditcard::en::*;
use fake::faker::internet::en::*;
use fake::{Dummy, Fake};
use rand::prelude::*;

// Defines `PRODUCT_IDS` and `CURRENCY_CODES` as slices of string literals.
include!(concat!(env!("OUT_DIR"), "/arrays.rs"));

#[derive(Debug, Dummy)]
pub struct Home {}

#[derive(Debug, Dummy)]
pub struct Product {
    #[dummy(faker = "ProductId")]
    pub id: String,
}

#[derive(Debug, Dummy)]
pub struct ViewCart {}

#[derive(Debug, Dummy)]
pub struct AddToCart {
    #[dummy(faker = "ProductId")]
    pub product_id: String,
    #[dummy(faker = "1..=10")]
    pub quantity: u32,
}

#[derive(Debug, Dummy)]
pub struct EmptyCart {}

#[derive(Debug, Dummy)]
pub struct SetCurrency {
    #[dummy(faker = "CurrencyCode")]
    pub currency_code: String,
}

#[derive(Debug, Dummy)]
pub struct Logout {}

#[derive(Debug, Dummy)]
pub struct PlaceOrder {
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
    #[dummy(faker = "StreetName()")]
    pub street_address: String,
    #[dummy(faker = "ZipCode()")]
    pub zip_code: String,
    #[dummy(faker = "CityName()")]
    pub city: String,
    #[dummy(faker = "StateAbbr()")]
    pub state: String,
    #[dummy(faker = "CountryName()")]
    pub country: String,
    #[dummy(faker = "Ccn")]
    pub cc_number: String,
    #[dummy(faker = "Month")]
    pub cc_month: String,
    #[dummy(faker = "Year")]
    pub cc_year: String,
    #[dummy(faker = "100..=999")]
    pub cc_cvv: u16,
}

struct ProductId;

impl Dummy<ProductId> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &ProductId, rng: &mut R) -> Self {
        PRODUCT_IDS.choose(rng).unwrap().to_string()
    }
}

struct CurrencyCode;

impl Dummy<CurrencyCode> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CurrencyCode, rng: &mut R) -> Self {
        CURRENCY_CODES.choose(rng).unwrap().to_string()
    }
}

struct Ccn;

impl Dummy<Ccn> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Ccn, rng: &mut R) -> Self {
        let ccn: String = CreditCardNumber().fake_with_rng(rng);
        ccn.as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .map(Result::unwrap)
            .collect::<Vec<&str>>()
            .join("-")
    }
}

struct Year;

impl Dummy<Year> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Year, rng: &mut R) -> Self {
        let current_year = Utc::now().year();
        rng.gen_range(current_year..=current_year + 5).to_string()
    }
}

struct Month;

impl Dummy<Month> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Month, rng: &mut R) -> Self {
        [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ]
        .choose(rng)
        .map(|&s| String::from(s))
        .unwrap()
    }
}
