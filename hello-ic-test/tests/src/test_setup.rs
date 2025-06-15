use std::path::PathBuf;

use candid::Principal;
use ic_test::{IcpTest, IcpUser};

use crate::bindings::hello_ic_test_backend::{self, HelloIcTestBackendCanister};

pub(crate) struct Env {
    pub icp_test: IcpTest,
    pub hello_ic_test_backend: HelloIcTestBackendCanister,
}

pub(crate) async fn setup(icp_test: IcpTest) -> Env {
    let icp_user = icp_test.icp.test_user(0);

    // initialize canisters

    let hello_ic_test_backend = hello_ic_test_backend::deploy(&icp_user, 50, 73)
        .call()
        .await;

    // Additional setup steps
    // ...

    Env {
        icp_test,
        hello_ic_test_backend,
    }
}
