use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ResultMode {
    Ignore,
    Save,
    Notify,
}

#[derive(Deserialize, Serialize)]
pub enum Filter {
    NameContains(String),
    PriceLower(i64),
}

#[derive(Deserialize, Serialize)]
pub struct FilterSteps {
    filter: Vec<Filter>,
    result: ResultMode,
}

#[derive(Deserialize, Serialize)]
pub struct Search {
    query: SearchQuery,
    filters: Vec<FilterSteps>,
    default: ResultMode,
}

#[derive(Deserialize, Serialize)]
pub struct SearchQuery {
    order: String,
    search_text: String,
    catalog_id: i64,
    #[serde(with = "commas")]
    brand_ids: Vec<i64>,
    #[serde(with = "commas")]
    size_ids: Vec<i64>,
    price_to: i64,
    currency: String,
    page: i64,
    per_page: i64,
}

mod commas {
    use std::str::FromStr;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serializer,
    };

    struct CommaList;

    impl<'de> Visitor<'de> for CommaList {
        type Value = Vec<i64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a comma-separated string")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let iter = s.split(",").map(FromStr::from_str);
            Result::from_iter(iter).map_err(Error::custom)
        }
    }

    pub fn serialize<S>(data: &Vec<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            &data
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }

    pub fn deserialize<'a, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_str(CommaList)
    }
}
