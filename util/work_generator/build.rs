use std::{fs, path::PathBuf};

use serde_json::Value;

const PRODUCTS: &str = include_str!("../../productcatalogservice/products.json");
const CURRENCY_CONVERSION: &str =
    include_str!("../../currencyservice/data/currency_conversion.json");

fn main() -> anyhow::Result<()> {
    let products: Value = serde_json::from_str(PRODUCTS)?;
    let product_ids: Vec<&str> = products
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Expected array"))?
        .iter()
        .map(|product| {
            let ids = product["id"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Expected string"))?;
            Ok(ids)
        })
        .collect::<anyhow::Result<_>>()?;
    let currency_conversion: Value = serde_json::from_str(CURRENCY_CONVERSION)?;
    let currency_codes: Vec<&str> = currency_conversion
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Expected object"))?
        .keys()
        .map(|k| k.as_str())
        .collect();
    let arrays = format!(
        "const PRODUCT_IDS: &[&str] = &{:?};\nconst CURRENCY_CODES: &[&str] = &{:?};",
        product_ids, currency_codes
    );
    let out = std::env::var("OUT_DIR")?;
    let path = PathBuf::from(out).join("arrays.rs");
    fs::write(path, arrays)?;
    Ok(())
}
