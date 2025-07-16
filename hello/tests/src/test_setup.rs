use std::path::PathBuf;

use candid::Principal;
use ic_test::{IcpTest, IcpUser};

use crate::bindings::hello_backend::{self, HelloBackendCanister};

pub(crate) struct Env {
    pub icp_test: IcpTest,
    pub hello_backend: HelloBackendCanister,
}

pub(crate) async fn setup(icp_test: IcpTest) -> Env {
    let icp_user = icp_test.icp.test_user(0);

    // initialize canisters

    let hello_backend = hello_backend::deploy(&icp_user).call().await;

    // Additional setup steps
    // ...

    Env {
        icp_test,
        hello_backend,
    }
}
