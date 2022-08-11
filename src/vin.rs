use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;

use self::responses::{
    data::{ItemDetails, SearchResult},
    VinItemResponse, VinPagedResponse,
};

pub struct VinRepository {
    api_root_url: String,
    client: reqwest::Client,
}
impl VinRepository {
    pub fn new(api_root_url: &str, cookie_value: &str) -> Self {
        VinRepository {
            api_root_url: api_root_url.to_string(),
            client: reqwest::Client::builder()
                .default_headers({
                    let mut h = HeaderMap::new();
                    h.insert("cookie", HeaderValue::from_str(cookie_value).unwrap());
                    h
                })
                .gzip(true)
                .build()
                .unwrap(),
        }
    }

    pub async fn search_by_query_string(
        &self,
        query: &str,
    ) -> Result<VinPagedResponse<SearchResult>, reqwest::Error> {
        self.endpoint_request(&format!("/catalog/items?{}", query))
            .await
    }

    pub async fn get_item_by_id(
        &self,
        id: &u64,
    ) -> Result<VinItemResponse<ItemDetails>, reqwest::Error> {
        self.endpoint_request(&format!("/items/{id}/details")).await
    }

    async fn endpoint_request<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, reqwest::Error> {
        self.client
            .execute(
                self.client
                    .get(format!("{}{}", self.api_root_url, &endpoint))
                    .build()?,
            )
            .await?
            .json()
            .await
    }
}

pub mod responses {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct VinPagedResponse<T> {
        items: Vec<T>,
        pagination: PaginationData,
    }

    #[derive(Debug, Deserialize)]
    pub struct PaginationData {
        current_page: u64,
        total_pages: u64,
        total_entries: u64,
        per_page: u64,
        time: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct VinItemResponse<T> {
        item: T,
    }

    pub mod data {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct SearchResult {
            id: u64,
            title: String,
            price: String,
            currency: String,
            brand_title: String,
            url: String,
            total_item_price: String,
            size_title: String,
            photo: Photo,
        }

        #[derive(Debug, Deserialize)]
        pub struct ItemDetails {
            id: u64,
            title: String,
            description: String,
            currency: String,
            total_item_price: String,
            size_title: String,
            photos: Vec<Photo>,
        }

        #[derive(Debug, Deserialize)]
        pub struct Photo {
            id: u64,
            url: String,
            full_size_url: String,
        }
    }
}
