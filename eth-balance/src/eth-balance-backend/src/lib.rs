use std::str::FromStr;

use alloy_primitives::U256;
use evm_rpc_canister_types::{EthMainnetService, RequestResult, RpcService, EVM_RPC};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonCall {
    id: u64,
    jsonrpc: String,
    method: String,
    params: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonResult {
    id: u64,
    jsonrpc: String,
    result: String,
}

#[ic_cdk::update]
async fn get_eth_balance(address: String) -> String {
    let get_balance_payload = JsonCall {
        jsonrpc: "2.0".to_string(),
        method: "eth_getBalance".to_string(),
        params: vec![address, "latest".to_string()],
        id: 1,
    };

    // connect to the real data node
    let provider = RpcService::EthMainnet(EthMainnetService::Llama);

    let json_text = serde_json::to_string(&get_balance_payload).expect("json serializer failed!");

    let cycles = 10_000_000_000;
    let response_size_limit = 3000;

    let result = EVM_RPC
        .request(provider, json_text, response_size_limit, cycles)
        .await;

    match result {
        Ok(r) => match r {
            (RequestResult::Ok(json_str),) => {
                let parsed_json: JsonResult = serde_json::from_str(&json_str)
                    .unwrap_or_else(|_| panic!("Failed parsing JsonResult from {json_str}!"));

                // convert result to a human-readable value
                let wei = U256::from_str_radix(parsed_json.result.trim_start_matches("0x"), 16)
                    .unwrap_or_else(|_| panic!("Invalid U256 from: {}", parsed_json.result));

                let eth_divisor = U256::from_str("1000000000000000000").unwrap();
                let eth_integer = wei / eth_divisor;
                let eth_fraction = wei % eth_divisor;

                let mut fraction_str = eth_fraction.to_string();
                while fraction_str.len() < 18 {
                    fraction_str.insert(0, '0');
                }
                let fraction_trimmed = fraction_str.trim_end_matches('0');

                if fraction_trimmed.is_empty() {
                    eth_integer.to_string()
                } else {
                    format!("{eth_integer}.{fraction_trimmed}")
                }
            }
            (RequestResult::Err(rpc_error),) => panic!("getBalance call failed: {rpc_error:?}"),
        },
        Err(e) => panic!("getBalance call failed! {e:?}"),
    }
}
