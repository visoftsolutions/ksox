// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.0;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
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
    mapping(address => bool) public isAccepted;
    address payable public wethAddress;
    address public soldTokenAddress;

    uint256 public prevAmountSold;
    uint256 public amountSold;
    mapping(address => uint256) public tokensCollected;
    uint256 public currentRateNumer;
    uint256 public currentRateDenom;
    uint256 public maxRateStepNumer;
    uint256 public maxRateStepDenom;
    uint256 public minimalAmountToSell;
    uint256 public minimalPrecreationTime;

    bool public isPhaseActive;
    uint256 public bucketStartTimestamp;
    uint256 public bucketFinishTimestamp;
    uint public currentBucketId;
    uint256 public currentBucketAmountToSell;
    bool public isBucketActive;

    event NewBucketCreated(
        uint256 indexed bucketId,
        uint indexed startTimestamp,
        uint indexed finishTimestamp,
        uint256 amountToSell,
        uint256 newRateNumer,
        uint256 newRateDenom
    );
    event BucketConcluded(
        uint256 indexed bucketId,
        uint256 amountSold,
        uint256 amountCollected
    );
    event BuyExecuted(
        address indexed buyer,
        address indexed token,
        uint256 amountIn,
        uint256 amountOut
    );
    event PhaseConcluded(uint256 amountSold);
    event WithdrawExecuted(
        address indexed token,
        address indexed to,
        uint256 amount
    );

    constructor(
        address _uniswapV3FactoryAddress,
        address _referenceTokenAddress,
        address payable _wethAddress,
        address[] memory _acceptedTokens,
        address _soldTokenAddress,
        uint256 _rateNumer,
        uint256 _rateDenom,
        uint256 _maxRateStepNumer,
        uint256 _maxRateStepDenom,
        uint256 _minimalAmountToSell,
        uint256 _minimalPrecreationTime
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
        currentRateNumer = _rateNumer;
        currentRateDenom = _rateDenom;
        maxRateStepNumer = _maxRateStepNumer;
        maxRateStepDenom = _maxRateStepDenom;
        minimalAmountToSell = _minimalAmountToSell;
        minimalPrecreationTime = _minimalPrecreationTime;

        prevAmountSold = 0;
        amountSold = 0;
        currentBucketId = 0;
        isPhaseActive = true;
    }

    function startNewBucket(
        uint256 startTimestamp,
        uint256 finishTimestamp,
        uint256 amountToSell,
        uint256 newRateNumer,
        uint256 newRateDenom
    ) public onlyOwner {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(!isBucketActive, "BUCKET_ACTIVE");
        require(
            startTimestamp >= minimalPrecreationTime
                && startTimestamp - minimalPrecreationTime >= block.timestamp,
            "INVALID_START_TIMESTAMP"
        );
        require(
            minimalAmountToSell >= amountToSell
                && amountToSell > 0,
            "INVALID_AMOUNT_TO_SELL"
        );
        require(
            // currentRate * maxRateStep > newRate && currentRate <= newRate
            currentRateNumer * maxRateStepNumer * newRateDenom > newRateNumer * maxRateStepDenom * currentRateDenom
                && newRateNumer * currentRateDenom >= newRateDenom * currentRateNumer,
            "INVALID_RATE"
        );
        bucketStartTimestamp = startTimestamp;
        bucketFinishTimestamp = finishTimestamp;
        currentBucketAmountToSell = amountToSell;
        currentRateNumer = newRateNumer;
        currentRateDenom = newRateDenom;
        isBucketActive = true;
        currentBucketId += 1;
        emit NewBucketCreated(
            currentBucketId,
            startTimestamp,
            finishTimestamp,
            amountToSell,
            newRateNumer,
            newRateDenom
        );
    }

    function concludeCurrentBucket() public onlyOwner {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(isBucketActive, "BUCKET_NOT_ACTIVE");
        _concludeCurrentBucket();
    }

    function _concludeCurrentBucket() internal {
        isBucketActive = false;
        uint256 currentBucketAmountSold = amountSold - prevAmountSold;
        prevAmountSold = amountSold;
        emit BucketConcluded(
            currentBucketId,
            amountSold,
            currentBucketAmountSold
        );
    }

    function getTokensCollected(address token) public view returns (uint256) {
        return tokensCollected[token];
    }

    function estimateAmountOut(
        address tokenIn,
        address tokenOut,
        uint24 fee,
        uint128 amountIn,
        uint24 secondsAgo
    ) internal view returns (uint256 amountOut) {
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

    function concludeWholePhase() public onlyOwner {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(!isBucketActive, "BUCKET_ACTIVE");
        isPhaseActive = false;
        emit PhaseConcluded(amountSold);
    }

    function withdraw(
        address token,
        address to,
        uint256 amount
    ) public onlyOwner {
        require(!isBucketActive, "BUCKET_ACTIVE");
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        IERC20(token).transfer(to, amount);
        emit WithdrawExecuted(token, to, amount);
    }

    function buyWithETH() public payable {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(wethAddress != address(0), "TOKEN_NOT_ACCEPTED");
        require(isBucketActive, "BUCKET_NOT_ACTIVE");
        require(msg.value > 0, "INVALID_AMOUNT");
        if (block.timestamp >= bucketFinishTimestamp) {
            msg.sender.transfer(msg.value);
            _concludeCurrentBucket();
        } else {
            address token = wethAddress;
            uint256 amount = msg.value;

            uint256 reducedAmount;
            uint256 reducedTokensToMint;
            (reducedAmount, reducedTokensToMint) = _calculateReducedAmounts(
                token,
                amount
            );
            IWETH(wethAddress).deposit{value: reducedAmount}();
            IWETH(wethAddress).transfer(msg.sender, reducedAmount);
            _buyWithERC20(
                msg.sender,
                address(this),
                wethAddress,
                reducedAmount,
                reducedTokensToMint
            );
            msg.sender.transfer(amount - reducedAmount);
            if (amount - reducedAmount > 0) {
                _concludeCurrentBucket();
            }

            emit BuyExecuted(msg.sender, token, amount, reducedTokensToMint);
        }
    }

    function buyWithERC20(address token, uint256 amount) public {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(isAccepted[token], "TOKEN_NOT_ACCEPTED");
        require(isBucketActive, "BUCKET_NOT_ACTIVE");
        require(amount > 0, "INVALID_AMOUNT");
        if (block.timestamp >= bucketFinishTimestamp) {
            _concludeCurrentBucket();
        } else {
            uint256 reducedAmount;
            uint256 reducedTokensToMint;
            (reducedAmount, reducedTokensToMint) = _calculateReducedAmounts(
                token,
                amount
            );
            _buyWithERC20(
                msg.sender,
                address(this),
                token,
                reducedAmount,
                reducedTokensToMint
            );
            if (amount - reducedAmount > 0) {
                _concludeCurrentBucket();
            }

            emit BuyExecuted(msg.sender, token, amount, reducedTokensToMint);
        }
    }

    function _calculateReducedAmounts(
        address token,
        uint256 amount
    )
        internal
        view
        returns (uint256 reducedAmount, uint256 reducedTokensToMint)
    {
        uint256 price = getERC20Price(token);
        uint256 tokensToMint = (amount * price * currentRateDenom) /
            10 ** IERC20Metadata(referenceTokenAddress).decimals() /
            currentRateNumer;
        reducedTokensToMint = min(
            tokensToMint,
            prevAmountSold + currentBucketAmountToSell - amountSold
        );
        reducedAmount = (amount * reducedTokensToMint) / tokensToMint;
    }

    function _buyWithERC20(
        address buyer,
        address seller,
        address token,
        uint256 amount,
        uint256 tokensToMint
    ) internal {
        IERC20(token).transferFrom(buyer, seller, amount);
        amountSold += tokensToMint;
        tokensCollected[token] += amount;
        IERC20Mintable(soldTokenAddress).mint(buyer, tokensToMint);
    }

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }
}
