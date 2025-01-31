use std::fs;

use candid::{CandidType, Decode, Encode, Principal};
use pocket_ic::{PocketIcBuilder, WasmResult};
use serde::Deserialize;

const SRC_WASM: &str = "test_can.wasm";

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

#[test]
fn test() {
    let pic = PocketIcBuilder::new().with_application_subnet().build();
    let cid = pic.create_canister();
    println!("cid: {}", cid.to_text());
    let src_wasm = fs::read(SRC_WASM).unwrap();
    pic.add_cycles(cid, 1_000 * 1_000_000_000_000);
    pic.install_canister(cid, src_wasm, vec![], None);
    //
    let arg = CanisterHttpRequestArgument {
        url: "http://example.com/".to_string(),
        max_response_bytes: Some(17),
        method: "POST".to_string(),
        headers: vec![],
        body: vec![42; 1_900_000],
        transform: Some(("ok".to_string(), "nok".to_string())),
    };
    let arg_bytes = Encode!(&arg).unwrap();
    let payload = Encode!(&arg_bytes).unwrap();
    println!("len encoded {}", arg_bytes.len());
    println!("len double encoded {}", payload.len());
    let res = pic
        .update_call(cid, Principal::anonymous(), "test", payload)
        .unwrap();
    let WasmResult::Reply(bytes) = res else {
        panic!("Expected Reply")
    };
    let res = Decode!(&bytes, u64);
    println!("res: {:?}", res);
}
