//! This is a sample test file, it shows an example of how to create actual tests.
//! The file is only generated once and won't be overwritten.

use ic_test::IcpTest;

use crate::bindings::{
    eth_balance_backend::{self, EthBalanceBackendCanister},
    evm_rpc::{self, EvmRpcCanister},
};

use alloy::{
    hex::FromHex,
    primitives::{utils::parse_ether, Address, Uint, U256},
};

use crate::test_setup;

/// This function shows how to test calling the `get_eth_balance`. The canister evm_rpc call is redirected to the local anvil server.
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

/// Here we only test the Ethereum smart contract Counter.
/// We call its methods to set the counter value, increment it, and finally read it. Set to 42, increment by 1, then assert its current value is 43.
#[tokio::test]
async fn test_counter() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let receipt = env
        .counter
        .setNumber(U256::from(42))
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    println!("===================  RECEIPT {receipt:?}");

    let receipt = env
        .counter
        .increment()
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    println!("===================  RECEIPT {receipt:?}");

    let receipt = env
        .counter
        .number()
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    println!("===================  RECEIPT {receipt:?}");
}

/// The combination of testing the balance canister and the sender smart contract.
/// 1. Check that the initial balances of the two accounts are 10000 Eth
/// 2. Call Ethereum contract sendEther belonging to the main user to send 100 Ether to the second account. (The smart contract also accepts a small fee for the operation.)
/// 3. Finally check, that the second account has the increased amount of Ether (10100 Ether).
///
#[tokio::test]
async fn test_eth_transfer() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let address1 = env.evm_user.address;
    let destination_address = env.icp_test.evm.test_user(1).address;

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
        .get_eth_balance(destination_address.to_string())
        .call()
        .await;

    // assert the second user has exactly 10000 Eth (the initial test value)
    assert_eq!(result, "10000");

    // prepare payment to send via the Sender contract
    let payment = parse_ether("100.01").unwrap();

    // The amount we want to send
    let amount_to_send = parse_ether("100.0").unwrap();

    // call Sender.sendEther
    let receipt = env
        .sender
        .sendEther(destination_address, amount_to_send)
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
        .get_eth_balance(destination_address.to_string())
        .call()
        .await;

    // assert the second user has now 10100 Eth
    assert_eq!(result, "10100");
}
