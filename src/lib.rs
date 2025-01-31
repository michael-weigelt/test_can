use candid::{CandidType, Decode};
use ic_cdk::{api::performance_counter, println, query};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct CanisterHttpRequestArgument {
    pub url: String,
    pub max_response_bytes: Option<u64>,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
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
