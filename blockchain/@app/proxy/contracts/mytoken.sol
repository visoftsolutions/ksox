// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

contract MyToken is Initializable, ERC20Upgradeable {
    address public owner;

    modifier onlyOwner() {
        require(owner == _msgSender(), "Not authorized");
        _;
    }

    function initialize(address initialOwner) initializer public {
        __ERC20_init("MyToken1000", "MTK1000");

        owner = initialOwner;
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}