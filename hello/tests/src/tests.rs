use std::path::PathBuf;

use candid::Principal;
use ic_test::{IcpTest, IcpUser};

use crate::bindings::hello_backend::{self, HelloBackendCanister};

use crate::test_setup;

#[tokio::test]
async fn test_greet() {
    let test_setup::Env {
        icp_test,
        hello_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    let result = hello_backend.greet_2("ic-test".to_string()).call().await;

    assert_eq!(result, "Hello, ic-test!");
}
