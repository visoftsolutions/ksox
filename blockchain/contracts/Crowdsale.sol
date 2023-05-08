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

contract Crowdsale is Ownable {
    struct Phase {
        uint256 soldAmount;
        uint256 maxAmount;
        mapping(address => uint256) tokensCollected;
    }

    address private factory;
    address private referenceToken;
    address payable private nativeWrappedToken;

    mapping(address => bool) isAccepted;

    address private soldToken;

    uint8 private currentPhaseId;

    // currentRate in cents, currentRate / 100
    uint256 private currentRate;

    // maxRateIncrease in promils, maxRateIncrease / 1000
    uint256 private maxRateIncrease;

    mapping(uint => Phase) phases;
    bool private isPhaseActive;
    bool private isCrowdsaleActive;

    constructor(
        address _factory,
        address _referenceToken,
        address payable _nativeWrappedToken,
        address[] memory _acceptedTokens,
        address _soldToken,
        uint256 _startRate,
        uint256 _maxRateIncrease
    ) {
        factory = _factory;
        referenceToken = _referenceToken;
        isAccepted[referenceToken] = true;
        nativeWrappedToken = _nativeWrappedToken;
        isAccepted[nativeWrappedToken] = true;
        for (uint i = 0; i < _acceptedTokens.length; i++) {
            isAccepted[_acceptedTokens[i]] = true;
        }
        soldToken = _soldToken;
        currentRate = _startRate;
        maxRateIncrease = _maxRateIncrease;
        isCrowdsaleActive = true;
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

        address pool = IUniswapV3Factory(factory).getPool(
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
        return
            estimateAmountOut(
                nativeWrappedToken,
                referenceToken,
                3000,
                uint128(
                    1 * 10 ** IERC20Metadata(nativeWrappedToken).decimals()
                ),
                3600
            );
    }

    function getERC20Price(address token) public view returns (uint256) {
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        return
            estimateAmountOut(
                token,
                referenceToken,
                3000,
                uint128(1 * 10 ** IERC20Metadata(token).decimals()),
                3600
            );
    }

    function startNewPhase(
        uint256 rate,
        uint256 amountToSell,
        uint8 numberOfBuckets
    ) public onlyOwner {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(!isPhaseActive, "PHASE_ALREADY_ACTIVE");
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(rate > 0, "INVALID_PRICE");
        require(amountToSell > 0, "INVALID_AMOUNT");
        require(numberOfBuckets > 0, "INVALID_BUCKETS");
        require(
            rate + (rate * maxRateIncrease) / 1000 < currentRate,
            "INVALID_PRICE"
        );
        require(
            amountToSell % numberOfBuckets == 0,
            "INVALID_NUMBER_OF_BUCKETS"
        );
        currentPhaseId += 1;
        phases[currentPhaseId].maxAmount = amountToSell;
        phases[currentPhaseId].soldAmount = 0;
        isPhaseActive = true;
    }

    function getLastPhaseMaxAmount() public view returns (uint256) {
        return phases[currentPhaseId].maxAmount;
    }

    function getLastPhaseSoldAmount() public view returns (uint256) {
        return phases[currentPhaseId].soldAmount;
    }

    function getLastPhaseTokensCollected(address token) public view returns (uint256) {
        return phases[currentPhaseId].tokensCollected[token];
    }

    function getPhaseTokensCollected(uint8 phaseId, address token) public view returns (uint256) {
        return phases[phaseId].tokensCollected[token];
    }

    function concludeCurrentPhase() public onlyOwner {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        isPhaseActive = false;
    }

    function withdraw(address token, address to) public onlyOwner {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(!isPhaseActive, "PHASE_ALREADY_ACTIVE");
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        IERC20(token).transfer(to, IERC20(token).balanceOf(address(this)));
    }

    function concludeCrowdsale() public onlyOwner {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        isCrowdsaleActive = false;
    }

    function buyWithETH() public payable {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        uint256 amount = msg.value;
        IWETH(nativeWrappedToken).deposit{value: amount}();
        uint256 price = getETHPrice();
        IERC20(nativeWrappedToken).transferFrom(
            msg.sender,
            address(this),
            amount
        );
        uint256 tokensToMint = amount * price * 100 / 10**IERC20Metadata(referenceToken).decimals() / currentRate;
        phases[currentPhaseId].soldAmount += tokensToMint;
        phases[currentPhaseId].tokensCollected[nativeWrappedToken] += amount;
        IERC20Mintable(soldToken).mint(msg.sender, tokensToMint);
    }

    function buyWithERC20(address token, uint256 amount) public {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        uint256 price = getERC20Price(token);
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        uint256 tokensToMint = amount * price * 100 / 10**IERC20Metadata(referenceToken).decimals() / currentRate;
        phases[currentPhaseId].soldAmount += tokensToMint;
        phases[currentPhaseId].tokensCollected[token] += amount;
        IERC20Mintable(soldToken).mint(msg.sender, tokensToMint);
    }

    function getCurrentPhase() public view returns (uint8) {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        return currentPhaseId;
    }

    function getCurrentRate() public view returns (uint256) {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        return currentRate;
    }

    function getCurrentPhaseMaxAmount() public view returns (uint256) {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        return phases[currentPhaseId].maxAmount;
    }

    function getCurrentPhaseSoldAmount() public view returns (uint256) {
        require(isCrowdsaleActive, "CROWDSALE_NOT_ACTIVE");
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        return phases[currentPhaseId].soldAmount;
    }
}
