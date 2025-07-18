// This is an experimental feature used to generate Rust bindings from Candid.
// THIS IS A GENERATED FILE. DO NOT EDIT THIS FILE TO AVOID DATA LOSS.
#![allow(dead_code, unused_imports, non_snake_case)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

pub struct HelloIcTestBackendCanister {
    pub canister_id: Principal,
    pub caller: super::Caller,
}

impl HelloIcTestBackendCanister {
    pub fn get_counter(&self) -> super::CallBuilder<u64> {
        let args = Encode!();
        self.caller.call(
            self.canister_id,
            super::CallMode::Query,
            "get_counter",
            args,
        )
    }
    pub fn greet(&self, arg0: String) -> super::CallBuilder<String> {
        let args = Encode!(&arg0);
        self.caller
            .call(self.canister_id, super::CallMode::Query, "greet", args)
    }
    pub fn increment_counter(&self) -> super::CallBuilder<()> {
        let args = Encode!();
        self.caller.call(
            self.canister_id,
            super::CallMode::Update,
            "increment_counter",
            args,
        )
    }
}

pub fn new(caller: &super::Caller, canister_id: Principal) -> HelloIcTestBackendCanister {
    HelloIcTestBackendCanister {
        canister_id,
        caller: caller.clone(),
    }
}

pub fn deploy(
    deployer: &super::Deployer,
    arg0: u64,
    arg1: u64,
) -> super::DeployBuilder<HelloIcTestBackendCanister> {
    let args = Encode!(&arg0, &arg1);
    let result = deployer.deploy(args, new);
    let result = if let Some(id) = canister_id() {
        result.with_canister_id(id)
    } else {
        result
    };
    if let Some(wasm) = wasm() {
        result.with_wasm(wasm)
    } else {
        result
    }
}
pub fn canister_id() -> Option<Principal> {
    None
}

pub fn wasm() -> Option<Vec<u8>> {
    let mut path = std::path::PathBuf::new();
    path.push("../.dfx/local/canisters/hello-ic-test-backend/hello-ic-test-backend.wasm");
    let wasm = std::fs::read(path.as_path())
        .unwrap_or_else(|_| panic!("wasm binary not found: {:?}", path));
    Some(wasm)
}
