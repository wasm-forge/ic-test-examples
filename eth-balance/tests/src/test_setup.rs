// This is a generated test setup file.
// Manual changes are possible, but you still need to make sure they are not lost, if the file is regenerated.
// If possible, it is best to keep any additional manual test preparation steps outside, in `tests.rs`,
// then this file can be regenerated without risk of losing work.

use ic_test::{EvmUser, IcpTest};

use crate::bindings::{
    eth_balance_backend::{self, EthBalanceBackendCanister},
    evm::Counter::{self, CounterInstance},
    evm::Sender::{self, SenderInstance},
    evm_rpc::{self, EvmRpcCanister},
};

pub(crate) struct Env {
    pub icp_test: IcpTest,
    pub eth_balance_backend: EthBalanceBackendCanister,
    pub evm_rpc: EvmRpcCanister,
    pub counter: CounterInstance<EvmUser, alloy::network::Ethereum>,
    pub sender: SenderInstance<EvmUser, alloy::network::Ethereum>,
    pub evm_user: EvmUser,
}

/// The function sets up testing environment, deploys all the canisters and
pub(crate) async fn setup(icp_test: IcpTest) -> Env {
    let evm_user = icp_test.evm.test_user(0);
    let icp_user = icp_test.icp.test_user(0);

    // initialize EVM contracts
    let counter = Counter::deploy(evm_user.clone()).await.unwrap();
    let sender = Sender::deploy(evm_user.clone()).await.unwrap();

    // initialize canisters

    let eth_balance_backend = eth_balance_backend::deploy(&icp_user).call().await;

    let evm_rpc = evm_rpc::deploy(
        &icp_user,
        evm_rpc::InstallArgs {
            logFilter: None,
            demo: None,
            manageApiKeys: None,
            overrideProvider: None,
            nodesInSubnet: None,
        },
    )
    .call()
    .await;

    Env {
        icp_test,
        evm_user,
        eth_balance_backend,
        evm_rpc,
        counter,
        sender,
    }
}
