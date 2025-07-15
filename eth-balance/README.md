# `eth-balance`

This is a sample eth-balance canister project. It has a single function "get_eth_balance" that for a given ETH address returns its current balance. The project uses the EVM-RPC canister to make the necessary RPC call.

To start dfx, run:
```bash
dfx start --clean --background
```

To deploy the project, run:

```bash
dfx deploy
```

To get ETH balance from the `vitalik.eth` address `0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045`, call:

```bash
dfx canister call eth-balance-backend get_eth_balance '("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")'
```

