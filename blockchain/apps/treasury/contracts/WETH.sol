// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/interfaces/IERC20.sol";

interface IWETH is IERC20 {
    event Deposit(address indexed from, uint256 value);
    event Withdrawal(address indexed to, uint256 value);

    receive() external payable;

    function deposit() external payable;

    function withdraw(uint256 wad) external;
}

contract WETH is IWETH, ERC20Permit {
    constructor(
        string memory _name,
        string memory _symbol
    ) ERC20(_name, _symbol) ERC20Permit(_name) {}

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
}
