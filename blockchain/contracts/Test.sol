// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.5.0 <0.8.0;

// Uncomment this line to use console.log
import "hardhat/console.sol";
import "./SimpleOracleLibrary.sol";

contract Test {

    function deposit(uint256 value) public payable {
        console.log("Received %s from %s", value, msg.sender);
        payable(msg.sender).transfer(value);
    }

    function withdraw() public {
        console.log("Withdrawn %s to %s", address(this).balance, msg.sender);
        payable(msg.sender).transfer(address(this).balance);
    }
}
