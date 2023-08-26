// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/interfaces/IERC20.sol";

interface IWETH is IERC20 {
    event Deposit(address indexed from, uint256 value);
    event Withdrawal(address indexed to, uint256 value);

    receive() external payable;

    function deposit() external payable;

    function withdraw(uint256 wad) external;
}
