use chrono::{Datelike, Utc};
use fake::faker::address::en::*;
use fake::faker::creditcard::en::*;
use fake::faker::internet::en::*;
use fake::{Dummy, Fake};
use rand::prelude::*;
use serde::Serialize;

// Defines `PRODUCT_IDS` and `CURRENCY_CODES` as slices of string literals.
include!(concat!(env!("OUT_DIR"), "/arrays.rs"));

#[derive(Debug, Dummy, Serialize)]
pub struct Home {}

#[derive(Debug, Dummy)]
pub struct Product {
    // NOTE: This is not a form value.
    #[dummy(faker = "ProductId")]
    pub id: String,
}

#[derive(Debug, Dummy, Serialize)]
pub struct ViewCart {}

#[derive(Debug, Dummy, Serialize)]
pub struct AddToCart {
    #[dummy(faker = "ProductId")]
    pub product_id: String,
    #[dummy(faker = "1..=10")]
    pub quantity: u32,
}

#[derive(Debug, Dummy, Serialize)]
pub struct EmptyCart {}

#[derive(Debug, Dummy, Serialize)]
pub struct SetCurrency {
    #[dummy(faker = "CurrencyCode")]
    pub currency_code: String,
}

#[derive(Debug, Dummy, Serialize)]
pub struct Logout {}

#[derive(Debug, Dummy, Serialize)]
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
    #[serde(rename = "credit_card_number")]
    pub cc_number: String,
    #[dummy(faker = "Month")]
    #[serde(rename = "credit_card_expiration_month")]
    pub cc_month: String,
    #[dummy(faker = "Year")]
    #[serde(rename = "credit_card_expiration_year")]
    pub cc_year: String,
    #[dummy(faker = "100..=999")]
    #[serde(rename = "credit_card_cvv")]
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
        let mut ccn: String = CreditCardNumber().fake_with_rng(rng);
        let first = if rng.gen_bool(0.5) { "4" } else { "5" }; // Visa or MasterCard
        ccn.replace_range(0..1, first);
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
        rng.gen_range(current_year + 1..=current_year + 5)
            .to_string()
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

#[cfg(test)]
#[cfg(ignore)]
mod tests {
    use fake::Faker;

    use super::*;

    #[tokio::test]
    async fn home() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let home: Home = Faker.fake();
        let status = client
            .get("http://34.49.117.79/")
            .header("Host", "onlineboutique.serviceweaver.dev")
            .form(&home)
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }

    #[tokio::test]
    async fn product() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let product: Product = Faker.fake();
        let status = client
            .get(format!("http://34.49.117.79/{}", product.id))
            .header("Host", "onlineboutique.serviceweaver.dev")
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }

    #[tokio::test]
    async fn view_cart() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let view_cart: ViewCart = Faker.fake();
        let status = client
            .get("http://34.49.117.79/cart")
            .header("Host", "onlineboutique.serviceweaver.dev")
            .form(&view_cart)
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }

    #[tokio::test]
    async fn add_to_cart() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let add_to_cart: AddToCart = Faker.fake();
        let status = client
            .post("http://34.49.117.79/cart")
            .header("Host", "onlineboutique.serviceweaver.dev")
            .form(&add_to_cart)
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }

    #[tokio::test]
    async fn empty_cart() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let empty_cart: EmptyCart = Faker.fake();
        let status = client
            .post("http://34.49.117.79/cart/empty")
            .header("Host", "onlineboutique.serviceweaver.dev")
            .header("Content-Length", "0") // required
            .form(&empty_cart)
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }

    #[tokio::test]
    async fn set_currency() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let set_currency: SetCurrency = Faker.fake();
        let status = client
            .post("http://34.49.117.79/setCurrency")
            .header("Host", "onlineboutique.serviceweaver.dev")
            .form(&set_currency)
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }

    #[tokio::test]
    async fn logout() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let logout: Logout = Faker.fake();
        let status = client
            .get("http://34.49.117.79/logout")
            .header("Host", "onlineboutique.serviceweaver.dev")
            .form(&logout)
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }

    #[tokio::test]
    async fn place_order() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let order: PlaceOrder = Faker.fake();
        let status = client
            .post("http://34.49.117.79/cart/checkout")
            .header("Host", "onlineboutique.serviceweaver.dev")
            .form(&order)
            .send()
            .await?
            .status();
        eprintln!("status = {:?}", status);
        assert!(status.is_success());
        Ok(())
    }
}
