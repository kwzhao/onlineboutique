use chrono::{Datelike, Utc};
use fake::faker::address::en::*;
use fake::faker::creditcard::en::*;
use fake::faker::internet::en::*;
use fake::{Dummy, Fake, Faker};
use rand::prelude::*;
use rand_distr::WeightedIndex;
use serde::{Deserialize, Serialize};

// Defines `PRODUCT_IDS` and `CURRENCY_CODES` as slices of string literals.
include!(concat!(env!("OUT_DIR"), "/arrays.rs"));

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct RequestMix {
    pub home: u64,
    pub product: u64,
    pub view_cart: u64,
    pub add_to_cart: u64,
    pub empty_cart: u64,
    pub set_currency: u64,
    pub logout: u64,
    pub place_order: u64,
}

impl Distribution<RequestData> for RequestMix {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RequestData {
        #[derive(Clone)]
        enum RequestKind {
            Home,
            Product,
            ViewCart,
            AddToCart,
            EmptyCart,
            SetCurrency,
            Logout,
            PlaceOrder,
        }
        let choices = &[
            (RequestKind::Home, self.home),
            (RequestKind::Product, self.product),
            (RequestKind::ViewCart, self.view_cart),
            (RequestKind::AddToCart, self.add_to_cart),
            (RequestKind::EmptyCart, self.empty_cart),
            (RequestKind::SetCurrency, self.set_currency),
            (RequestKind::Logout, self.logout),
            (RequestKind::PlaceOrder, self.place_order),
        ];
        let (kinds, weights) = choices
            .iter()
            .cloned()
            .unzip::<RequestKind, u64, Vec<_>, Vec<_>>();
        let index_dist = WeightedIndex::new(weights).unwrap();
        let kind = &kinds[index_dist.sample(rng)];
        match kind {
            RequestKind::Home => RequestData::Home(Faker.fake()),
            RequestKind::Product => RequestData::Product(Faker.fake()),
            RequestKind::ViewCart => RequestData::ViewCart(Faker.fake()),
            RequestKind::AddToCart => RequestData::AddToCart(Faker.fake()),
            RequestKind::EmptyCart => RequestData::EmptyCart(Faker.fake()),
            RequestKind::SetCurrency => RequestData::SetCurrency(Faker.fake()),
            RequestKind::Logout => RequestData::Logout(Faker.fake()),
            RequestKind::PlaceOrder => RequestData::PlaceOrder(Faker.fake()),
        }
    }
}

#[derive(Debug)]
pub enum RequestData {
    Home(Home),
    Product(Product),
    ViewCart(ViewCart),
    AddToCart(AddToCart),
    EmptyCart(EmptyCart),
    SetCurrency(SetCurrency),
    Logout(Logout),
    PlaceOrder(PlaceOrder),
}

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
// #[cfg(_ignore)]
mod tests {
    use fake::Faker;
    use reqwest::Url;

    use super::*;

    fn base_url() -> Url {
        "http://localhost:62662".parse().unwrap()
    }

    #[tokio::test]
    async fn home() -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let home: Home = Faker.fake();
        let status = client
            .get(base_url())
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
            .get(format!("{}{}", base_url(), product.id))
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
            .get(format!("{}cart", base_url()))
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
            .post(format!("{}cart", base_url()))
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
            .post(format!("{}cart/empty", base_url()))
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
            .post(format!("{}setCurrency", base_url()))
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
            .get(format!("{}logout", base_url()))
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
            .post(format!("{}cart/checkout", base_url()))
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
