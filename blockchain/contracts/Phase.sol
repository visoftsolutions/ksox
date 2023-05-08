// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.0;

// Uncomment this line to use console.log
import "hardhat/console.sol";
import "@uniswap/v3-core/contracts/libraries/FullMath.sol";
import "@uniswap/v3-core/contracts/libraries/TickMath.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol";
import "./Ownable.sol";
import "./IERC20Metadata.sol";
import "./IERC20Mintable.sol";
import "./SimpleOracleLibrary.sol";
import "./IWETH.sol";

contract Phase is Ownable {
    address public uniswapV3FactoryAddress;
    address public referenceTokenAddress;
    address payable public wethAddress;
    address public soldTokenAddress;

    uint256 public maxSellTokenAmount;
    uint256 public soldAmount;
    mapping(address => uint256) public tokensCollected;
    mapping(address => bool) public isAccepted;

    uint256 public currentRate;
    uint256 public currentRateDenominator;
    bool public isPhaseActive;

    constructor(
        address _uniswapV3FactoryAddress,
        address _referenceTokenAddress,
        address payable _wethAddress,
        address[] memory _acceptedTokens,
        address _soldTokenAddress,
        uint256 _startRate,
        uint256 _maxSellTokenAmount
    ) {
        uniswapV3FactoryAddress = _uniswapV3FactoryAddress;
        referenceTokenAddress = _referenceTokenAddress;
        isAccepted[referenceTokenAddress] = true;
        wethAddress = _wethAddress;
        isAccepted[wethAddress] = true;
        for (uint i = 0; i < _acceptedTokens.length; i++) {
            isAccepted[_acceptedTokens[i]] = true;
        }
        soldTokenAddress = _soldTokenAddress;
        currentRate = _startRate;
        currentRateDenominator = 100;
        maxSellTokenAmount = _maxSellTokenAmount;
        isPhaseActive = true;
    }

    function estimateAmountOut(
        address tokenIn,
        address tokenOut,
        uint24 fee,
        uint128 amountIn,
        uint24 secondsAgo
    ) public view returns (uint256 amountOut) {
        require(isAccepted[tokenIn], "TOKEN_NOT_ACCEPTED");
        require(isAccepted[tokenOut], "TOKEN_NOT_ACCEPTED");

        address pool = IUniswapV3Factory(uniswapV3FactoryAddress).getPool(
            tokenIn,
            tokenOut,
            fee
        );
        int24 tick = SimpleOracleLibrary.consult(pool, secondsAgo);
        amountOut = SimpleOracleLibrary.getQuoteAtTick(
            tick,
            amountIn,
            tokenIn,
            tokenOut
        );
    }

    function getETHPrice() public view returns (uint256) {
        require(wethAddress != address(0), "TOKEN_NOT_ACCEPTED");
        return
            estimateAmountOut(
                wethAddress,
                referenceTokenAddress,
                3000,
                uint128(1 * 10 ** IERC20Metadata(wethAddress).decimals()),
                3600
            );
    }

    function getERC20Price(address token) public view returns (uint256) {
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        return
            estimateAmountOut(
                token,
                referenceTokenAddress,
                3000,
                uint128(1 * 10 ** IERC20Metadata(token).decimals()),
                3600
            );
    }

    function concludeCurrentPhase() public onlyOwner {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        isPhaseActive = false;
    }

    function withdraw(address token, address to) public onlyOwner {
        require(!isPhaseActive, "PHASE_ALREADY_ACTIVE");
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        IERC20(token).transfer(to, IERC20(token).balanceOf(address(this)));
    }

    function buyWithETH() public payable {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(wethAddress != address(0), "TOKEN_NOT_ACCEPTED");
        uint256 amount = msg.value;
        IWETH(wethAddress).deposit{value: amount}();
        uint256 price = getETHPrice();
        IERC20(wethAddress).transferFrom(msg.sender, address(this), amount);
        uint256 tokensToMint = (amount * price * currentRateDenominator) /
            10 ** IERC20Metadata(referenceTokenAddress).decimals() /
            currentRate;
        soldAmount += tokensToMint;
        tokensCollected[wethAddress] += amount;
        IERC20Mintable(soldTokenAddress).mint(msg.sender, tokensToMint);
    }

    function buyWithERC20(address token, uint256 amount) public {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        uint256 price = getERC20Price(token);
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        uint256 tokensToMint = (amount * price * currentRateDenominator) /
            10 ** IERC20Metadata(referenceTokenAddress).decimals() /
            currentRate;
        soldAmount += tokensToMint;
        tokensCollected[token] += amount;
        IERC20Mintable(soldTokenAddress).mint(msg.sender, tokensToMint);
    }
}
