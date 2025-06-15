use std::path::PathBuf;

use candid::Principal;
use ic_test::{IcpTest, IcpUser};

use crate::bindings::ic_rusqlite_backend::{self, IcRusqliteBackendCanister};

use crate::test_setup;

#[tokio::test]
async fn test_create_table_and_get_element_count() {
    let test_setup::Env {
        icp_test,
        ic_rusqlite_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    // example calls
    ic_rusqlite_backend
        .add("Name 1".to_string(), "Info 1".to_string(), 32)
        .call()
        .await;

    ic_rusqlite_backend
        .add("Name 2".to_string(), "Info 2".to_string(), 33)
        .call()
        .await;

    ic_rusqlite_backend
        .add("Name 3".to_string(), "Info 3".to_string(), 34)
        .call()
        .await;

    let result = ic_rusqlite_backend
        .query("SELECT count(*) FROM person".to_string())
        .call()
        .await;

    match result {
        ic_rusqlite_backend::Result_::Ok(items) => {
            let count = items.first().unwrap().first().unwrap();

            assert_eq!(count, "3");
        }
        ic_rusqlite_backend::Result_::Err(error) => panic!("Error during canister launch"),
    }
}
