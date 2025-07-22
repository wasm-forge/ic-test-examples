// THIS IS A GENERATED FILE, DO NOT EDIT!
#![allow(dead_code, unused_imports, non_snake_case)]

type CallMode = ic_test::CallMode;
type Caller = ic_test::IcpUser;
type CallBuilder<R> = ic_test::CallBuilder<R, ic_test::IcpUser>;
type DeployMode = ic_test::DeployMode;
type Deployer = ic_test::IcpUser;
type DeployBuilder<C> = ic_test::DeployBuilder<C, Caller>;

// candid: src/eth-balance-backend/eth-balance-backend.did
pub mod eth_balance_backend;

// candid: .dfx/local/canisters/evm_rpc/constructor.did
pub mod evm_rpc;


pub mod evm {
    use alloy::sol;
    
    sol!(
        #[sol(rpc)]
        Counter,
        ".././evm/out/Counter.sol/Counter.json",
    );
    
    sol!(
        #[sol(rpc)]
        Sender,
        ".././evm/out/Sender.sol/Sender.json",
    );
    
}
