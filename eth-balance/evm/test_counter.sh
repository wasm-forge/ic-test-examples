#!/bin/bash

# launch Anvil
# Find process IDs listening on port 8545 (anvil)
anvil=$(lsof -t -i:8545)

# Check if any PIDs were found
if [ -z "$anvil" ]; then
    echo "Starting anvil..."
    anvil
    sleep 2
fi

# First account (default by anvil)
export ACCOUNT1=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
# Private key for the first account
export PRIVATE_KEY1=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# Second account (default by anvil)
export ACCOUNT2=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
# Private key for the second account
export PRIVATE_KEY2=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

# Local Anvil instance
export RPC_URL=http://127.0.0.1:8545

# Deploy Counter contract by the first account
forge create src/Counter.sol:Counter --rpc-url $RPC_URL --private-key $PRIVATE_KEY1 --broadcast
# Deploy Sender contract by the first account
forge create src/Sender.sol:Sender --rpc-url $RPC_URL --private-key $PRIVATE_KEY1 --broadcast

# Contract IDs are known from the deployer and the deployer's nonce
export COUNTER=$(cast compute-address --nonce 0 $ACCOUNT1 | awk '{ print $3 }')
export SENDER=$(cast compute-address --nonce 1 $ACCOUNT1 | awk '{ print $3 }')


# set counter to 42
cast send $COUNTER "setNumber(uint256)" 42 --private-key $PRIVATE_KEY1 --rpc-url $RPC_URL

# call counter increment
cast send $CONTRACT "increment()" --private-key $PRIVATE_KEY1 --rpc-url $RPC_URL

# get counter number, should return 43
export ANSWER=$(cast call $CONTRACT "number()(uint256)" --rpc-url $RPC_URL)

echo "Final answer: $ANSWER"
