// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Context.sol";
import "solady/src/tokens/ERC20.sol";

contract Treasury is Context {
    
    function requestDeposit(address token, uint256 amount) public {}
    function requestWithdraw() public {}
}
