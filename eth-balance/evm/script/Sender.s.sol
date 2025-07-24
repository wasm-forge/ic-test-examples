// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Sender} from "../src/Sender.sol";

contract SenderScript is Script {
    Sender public sender;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        sender = new Sender();

        vm.stopBroadcast();
    }
}
