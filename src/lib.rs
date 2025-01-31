use candid::{CandidType, Decode};
use ic_cdk::{api::performance_counter, println, query};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct CanisterHttpRequestArgument {
    pub url: String,
    pub max_response_bytes: Option<u64>,
    pub method: String,
    pub headers: Vec<(String, String)>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
    pub transform: Option<(String, String)>,
}

#[query]
fn test(arg: Vec<u8>) -> u64 {
    let start = performance_counter(0);
    let decoded = Decode!(&arg, CanisterHttpRequestArgument).unwrap();
    // println!("{:?}", arg);
    let end = performance_counter(0);
    end - start
}

mod deserialize {
    /// Efficient deserializer for `Option<Vec<u8>>` using `serde_bytes::ByteBuf` internally
    /// to speed up deserialization.
    pub fn deserialize_option_blob<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;

        let s: Option<serde_bytes::ByteBuf> = Option::deserialize(deserializer)?;
        Ok(s.map(|b| b.to_vec()))
    }

    /// Efficient deserializer for `Vec<Vec<u8>>` using `serde_bytes::ByteBuf` internally
    /// for the inner vector to speed up deserialization.
    pub fn deserialize_vec_blob<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;

        let s: Vec<serde_bytes::ByteBuf> = Vec::deserialize(deserializer)?;
        Ok(s.into_iter().map(|b| b.to_vec()).collect())
    }
}
