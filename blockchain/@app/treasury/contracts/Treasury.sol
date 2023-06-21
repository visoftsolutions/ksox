// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/interfaces/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Phase is Ownable {
    mapping(address => mapping(address => uint256)) private _balances;

    event Deposit(
        address indexed tokenAddress,
        address indexed userAddress,
        uint256 amount
    );
    event Withdraw(
        address indexed tokenAddress,
        address indexed userAddress,
        uint256 amount
    );
    event SetBalance(
        address indexed tokenAddress,
        address indexed userAddress,
        uint256 value
    );

    constructor() {}
    
    function balanceOf(address tokenAddress, address userAddress) public view returns (uint256) {
        return _balances[tokenAddress][userAddress];
    }

    function deposit(address tokenAddress, uint256 amount) external {
        address userAddress = address(msg.sender);
        IERC20(tokenAddress).transferFrom(userAddress, address(this), amount);
        _balances[tokenAddress][userAddress] += amount;
        emit Deposit(tokenAddress, userAddress, amount);
    }

    function depositPermit(address tokenAddress, uint256 amount, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external {
        address owner = address(msg.sender);
        IERC20Permit(tokenAddress).permit(owner, address(this), amount, deadline, v, r, s);
        IERC20(tokenAddress).transferFrom(owner, address(this), amount);
        _balances[tokenAddress][owner] += amount;
        emit Deposit(tokenAddress, owner, amount);
    }

    function withdraw(address tokenAddress, address userAddress, uint256 amount) external onlyOwner {
        require(_balances[tokenAddress][userAddress] >= amount);
        _balances[tokenAddress][userAddress] -= amount;
        IERC20(tokenAddress).transfer(userAddress, amount);
        emit Withdraw(tokenAddress, userAddress, amount);
    }

    function setBalance(address tokenAddress, address userAddress, uint256 value) external onlyOwner {
        _balances[tokenAddress][userAddress] = value;
        emit SetBalance(tokenAddress, userAddress, value);
    }
}