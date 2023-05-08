// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import "solady/src/tokens/ERC20.sol";
import "solady/src/auth/Ownable.sol";

contract Token is ERC20, Ownable {
    constructor() {
        _mint(msg.sender, 100 * 10**decimals());
    }
    
    function name() public pure override returns (string memory) {
        return "KSOX";
    }
    function symbol() public pure override returns (string memory) {
        return "KSX";
    }
    function mint(address to, uint256 amount) public onlyOwner returns (bool) {
        _mint(to, amount);
        return true;
    }
}
