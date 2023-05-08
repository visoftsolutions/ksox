// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import "solady/src/tokens/ERC20.sol";
import "solady/src/auth/OwnableRoles.sol";

contract TokenTicket is ERC20 {
    
    function name() public pure override returns (string memory) {
        return "KSOX Ticket Token";
    }
    function symbol() public pure override returns (string memory) {
        return "KSXT";
    }
}
