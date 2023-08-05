// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/interfaces/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Treasury is Ownable {
    mapping(address => mapping(address => uint256)) private _balances;

    struct BalanceUpdate {
        address tokenAddress;
        address userAddress;
        uint256 value;
    }

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
    event SetBalances(
        BalanceUpdate[] indexed updates
    );

    constructor() {}
    
    function balanceOf(address tokenAddress, address userAddress) public view returns (uint256) {
        return _balances[tokenAddress][userAddress];
    }

    function deposit(address tokenAddress, uint256 amount) external {
        address owner = address(msg.sender);
        IERC20(tokenAddress).transferFrom(owner, address(this), amount);
        _balances[tokenAddress][owner] += amount;
        emit Deposit(tokenAddress, owner, amount);
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

    function _setBalance(BalanceUpdate memory update) internal {
        _balances[update.tokenAddress][update.userAddress] = update.value;
    }

    function setBalances(BalanceUpdate[] memory updates) external onlyOwner {
        for (uint256 i = 0; i < updates.length; i++) {
           _setBalance(updates[i]);
        }
        emit SetBalances(updates);
    }
}