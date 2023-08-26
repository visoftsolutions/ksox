// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "./IWETH.sol";

contract WETH is IWETH, AccessControl, ERC20Permit {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    constructor(
        string memory _name,
        string memory _symbol
    ) ERC20(_name, _symbol) ERC20Permit(_name) {
        _grantRole(DEFAULT_ADMIN_ROLE, _msgSender());
        _grantRole(MINTER_ROLE, _msgSender());
    }

    receive() external payable {
        deposit();
    }

    function deposit() public payable {
        _mint(_msgSender(), msg.value);
        emit Deposit(_msgSender(), msg.value);
    }

    function withdraw(uint256 amount) public {
        require(balanceOf(_msgSender()) >= amount);
        _burn(_msgSender(), amount);
        payable(_msgSender()).transfer(amount);
        emit Withdrawal(_msgSender(), amount);
    }

    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
        _mint(to, amount);
    }
}
