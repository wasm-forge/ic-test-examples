//! This is a sample test file, it shows an example of how to create actual tests.
//! The file is only generated once and won't be overwritten.

use ic_test::IcpTest;

use crate::bindings::{
    eth_balance_backend::{self, EthBalanceBackendCanister},
    evm::Counter::{self, CounterInstance},
    evm_rpc::{self, EvmRpcCanister},
};

use alloy::{
    hex::FromHex,
    primitives::{utils::parse_ether, Address, Uint, U256},
};

use crate::test_setup;

/// This is an example test function. It shows how to create a test environment and use it to call canister methods.
/// Update it to actually do testing.
#[tokio::test]
async fn test_eth_get_balance() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let address1 = env.evm_user.address.to_string();
    let address2 = env.icp_test.evm.test_user(1).address.to_string();

    let result = env
        .eth_balance_backend
        .get_eth_balance(address1.to_string())
        .call()
        .await;

    let eth = parse_ether(&result).unwrap();

    // assert the main user still has around 10000 Ether after deploying the smart contracts
    assert!(parse_ether("10000").unwrap() - eth < parse_ether("0.01").unwrap());

    let result = env
        .eth_balance_backend
        .get_eth_balance(address2)
        .call()
        .await;

    // assert the second user has exactly 10000 Eth (the initial test value)
    assert_eq!(result, "10000");
}

#[tokio::test]
async fn test_eth_transfer() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let address1 = env.evm_user.address;
    let address2 = env.icp_test.evm.test_user(1).address;

    let result = env
        .eth_balance_backend
        .get_eth_balance(address1.to_string())
        .call()
        .await;

    let eth = parse_ether(&result).unwrap();

    // assert the main user still has around 10000 Ether after deploying contracts
    assert!(parse_ether("10000").unwrap() - eth < parse_ether("0.01").unwrap());

    let result = env
        .eth_balance_backend
        .get_eth_balance(address2.to_string())
        .call()
        .await;

    // assert the second user has exactly 10000 Eth (the initial test value)
    assert_eq!(result, "10000");

    // prepare payment to send via the Sender contract
    let payment = parse_ether("100.01").unwrap();

    // The amount we want to send
    let to_send = parse_ether("100.0").unwrap();

    // call Sender.sendEther
    let receipt = env
        .sender
        .sendEther(address2, to_send)
        .value(payment)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    assert!(receipt.status());

    let result = env
        .eth_balance_backend
        .get_eth_balance(address2.to_string())
        .call()
        .await;

    // assert the second user has now 10100 Eth
    assert_eq!(result, "10100");
}
