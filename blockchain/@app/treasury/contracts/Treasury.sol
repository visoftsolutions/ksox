// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/interfaces/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract Treasury is Ownable, EIP712 {
  address private publicKey;
  string private name_;

  using Counters for Counters.Counter;

  mapping(address => Counters.Counter) private _nonces;

  // solhint-disable-next-line var-name-mixedcase
  bytes32 private constant _PERMIT_TYPEHASH = keccak256("Permit(address owner,address spender,address token,uint256 value,uint256 nonce,uint256 deadline)");

  event Deposit(address indexed owner, address indexed spender, address indexed token, uint256 amount);
  event Withdraw(address indexed owner, address indexed spender, address indexed token, uint256 amount, uint256 nonce, uint256 deadline);

  constructor(string memory _name, address _publicKey) EIP712(_name, "1") {
    publicKey = _publicKey;
    name_ = _name;
  }

  /**
   * @dev Returns the name of the contract.
   */
  function name() public view returns (string memory) {
    return name_;
  }

  /**
   * @dev See {IERC20Permit-nonces}.
   */
  function nonces(address owner) public view returns (uint256) {
    return _nonces[owner].current();
  }

  /**
   * @dev See {IERC20Permit-DOMAIN_SEPARATOR}.
   */
  // solhint-disable-next-line func-name-mixedcase
  function DOMAIN_SEPARATOR() external view returns (bytes32) {
    return _domainSeparatorV4();
  }

  /**
   * @dev "Consume a nonce": return the current value and increment.
   *
   * _Available since v4.1._
   */
  function _useNonce(address owner) internal returns (uint256 current) {
    Counters.Counter storage nonce = _nonces[owner];
    current = nonce.current();
    nonce.increment();
  }

  function depositPermit(address token, uint256 amount, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external {
    address owner = address(msg.sender);
    IERC20Permit(token).permit(owner, address(this), amount, deadline, v, r, s);
    IERC20(token).transferFrom(owner, address(this), amount);
    emit Deposit(owner, publicKey, token, amount);
  }

  function withdrawPermit(address token, uint256 amount, uint256 deadline, uint8 v, bytes32 r, bytes32 s, address to) external {
    address owner = address(msg.sender);
    uint256 nonce = _useNonce(owner);

    require(block.timestamp <= deadline, "ERC20Permit: expired deadline");

    bytes32 structHash = keccak256(abi.encode(_PERMIT_TYPEHASH, publicKey, owner, token, amount, nonce, deadline));

    bytes32 hash = _hashTypedDataV4(structHash);

    address signer = ECDSA.recover(hash, v, r, s);
    require(signer == publicKey, "ERC20Permit: invalid signature");

    IERC20(token).transfer(to, amount);

    emit Withdraw(publicKey, owner, token, amount, nonce, deadline);
  }
}
