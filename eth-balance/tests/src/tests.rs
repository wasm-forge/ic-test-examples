use ic_test::IcpTest;

use crate::bindings::{
    eth_balance_backend::{self, EthBalanceBackendCanister},
    evm::Counter::{self, CounterInstance},
    evm_rpc::{self, EvmRpcCanister},
};

use crate::test_setup;

/// This is an example test function. It shows how to create a test environment and use it to call canister methods.
/// Update it to actually do testing.
#[tokio::test]
async fn test_() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let address = env.evm_user.address;

    let result = env
        .eth_balance_backend
        .get_eth_balance(address.to_string())
        .call()
        .await;

    println!("result = {result}");

    // ...
}
