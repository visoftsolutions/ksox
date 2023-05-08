// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.5.0 <0.8.0;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import "./IERC20.sol";

interface IERC20Mintable is IERC20 {
    function mint(address to, uint256 amount) external returns (bool);
}
