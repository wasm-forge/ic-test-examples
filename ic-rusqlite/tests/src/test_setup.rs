use std::path::PathBuf;

use candid::Principal;
use ic_test::{IcpTest, IcpUser};

use crate::bindings::ic_rusqlite_backend::{self, IcRusqliteBackendCanister};

pub(crate) struct Env {
    pub icp_test: IcpTest,
    pub ic_rusqlite_backend: IcRusqliteBackendCanister,
}

pub(crate) async fn setup(icp_test: IcpTest) -> Env {
    let icp_user = icp_test.icp.test_user(0);

    // initialize canisters

    let ic_rusqlite_backend = ic_rusqlite_backend::deploy(&icp_user).call().await;

    // Additional setup steps
    // ...

    Env {
        icp_test,
        ic_rusqlite_backend,
    }
}
