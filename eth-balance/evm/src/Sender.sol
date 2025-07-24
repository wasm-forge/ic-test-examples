// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Sender {
    address public owner;

    constructor() {
        owner = msg.sender; // Set the contract deployer as the owner
    }

    // Function to receive Ether. Required for the contract to accept ETH.
    receive() external payable {}

    // Send Ether from the contract to a recipient, the function is payable so that we can send the money along the call
    function sendEther(address payable receiver, uint256 eth) external payable {
        require(msg.sender == owner, "Only owner can send Ether");
        require(address(this).balance >= eth, "Insufficient balance");

        (bool sent, ) = receiver.call{value: eth}("");
        require(sent, "Failed to send Ether");
    }

    // Check contract balance
    function getBalance() external view returns (uint256) {
        return address(this).balance;
    }
}