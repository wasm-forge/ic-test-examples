use ic_test::IcpTest;

use crate::test_setup;

#[tokio::test]
async fn test_greet() {
    let test_setup::Env {
        icp_test,
        hello_ic_test_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    let result = hello_ic_test_backend
        .greet("ic-test".to_string())
        .call()
        .await;

    assert_eq!(result, "Hello, ic-test!");
}

#[tokio::test]
async fn test_counter() {
    let test_setup::Env {
        icp_test,
        hello_ic_test_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    let result = hello_ic_test_backend.get_counter().call().await;

    assert_eq!(result, 50u64);

    hello_ic_test_backend.increment_counter().call().await;

    let result = hello_ic_test_backend.get_counter().call().await;

    assert_eq!(result, 123u64); // 50 + 73
}
