// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.5.0 <0.8.0;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import "./IERC20.sol";

interface IWETH is IERC20 {
    receive() external payable;

    function deposit() external payable;

    function withdraw(uint256 wad) external;
}
