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
    event BucketCreated(
        uint256 indexed bucketId,
        uint256 indexed startTimestamp,
        uint256 indexed endTimestamp,
        uint256 capacity,
        uint256 rateNumer,
        uint256 rateDenom
    );
    event BuyExecuted(uint256 indexed bucketId, uint256 bucketSoldAmount);
    event BucketConcluded(uint256 indexed bucketId, uint256 bucketSoldAmount);
    event PhaseConcluded(uint256 totalSoldAmount);
    event WithdrawExecuted(
        address indexed tokenAddress,
        address indexed toAddress,
        uint256 amount
    );

    // Phase State
    string public name;
    address public immutable uniswapV3FactoryAddress;
    address public immutable referenceTokenAddress;
    address payable public immutable wethAddress;
    address public immutable soldTokenAddress;
    uint256 public immutable minBucketStartDelay;
    uint256 public immutable minBucketDuration;
    uint256 public immutable minBucketCapacity;
    uint256 public immutable maxBucketRateIncreaseNumer;
    uint256 public immutable maxBucketRateIncreaseDenom;

    mapping(address => bool) public isTokenAccepted;
    uint256 public totalSoldAmount;
    mapping(address => uint256) public totalTokensCollected;
    bool public isPhaseActive;
    bool public isBucketActive;

    uint256 public immutable tenToTheReferenceTokenDecimals;
    uint256 public immutable tenToTheWethTokenDecimals;

    // Current Bucket
    uint256 public currentBucketId;
    uint256 public currentBucketStartTimestamp;
    uint256 public currentBucketEndTimestamp;
    uint256 public currentBucketCapacity;
    uint256 public currentBucketRateNumer;
    uint256 public currentBucketRateDenom;
    uint256 public currentBucketSoldAmount;

    constructor(
        string memory _name,
        address _uniswapV3FactoryAddress,
        address _referenceTokenAddress,
        address payable _wethAddress,
        address _soldTokenAddress,
        uint256 _minBucketStartDelay,
        uint256 _minBucketDuration,
        uint256 _minBucketCapacity,
        uint256 _maxBucketRateIncreaseNumer,
        uint256 _maxBucketRateIncreaseDenom,
        address[] memory _acceptedTokens,
        uint256 _initialRateNumer,
        uint256 _initialRateDenom
    ) {
        name = _name;
        uniswapV3FactoryAddress = _uniswapV3FactoryAddress;
        referenceTokenAddress = _referenceTokenAddress;
        wethAddress = _wethAddress;
        soldTokenAddress = _soldTokenAddress;
        minBucketStartDelay = _minBucketStartDelay;
        minBucketDuration = _minBucketDuration;
        minBucketCapacity = _minBucketCapacity;
        maxBucketRateIncreaseNumer = _maxBucketRateIncreaseNumer;
        maxBucketRateIncreaseDenom = _maxBucketRateIncreaseDenom;

        isTokenAccepted[_referenceTokenAddress] = true;
        isTokenAccepted[_wethAddress] = true;
        for (uint i = 0; i < _acceptedTokens.length; i++) {
            isTokenAccepted[_acceptedTokens[i]] = true;
        }

        tenToTheReferenceTokenDecimals =
            10 ** IERC20Metadata(_referenceTokenAddress).decimals();
        tenToTheWethTokenDecimals =
            10 ** IERC20Metadata(_wethAddress).decimals();

        currentBucketId = 0;
        currentBucketRateNumer = _initialRateNumer;
        currentBucketRateDenom = _initialRateDenom;

        isPhaseActive = true;
    }

    function startNewBucket(
        uint256 startTimestamp,
        uint256 endTimestamp,
        uint256 capacity,
        uint256 newRateNumer,
        uint256 newRateDenom
    ) public onlyOwner {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(!isBucketActive, "BUCKET_ACTIVE");
        require(
            startTimestamp >= minBucketStartDelay &&
                startTimestamp - minBucketStartDelay >= block.timestamp,
            "INVALID_START_TIMESTAMP"
        );
        require(capacity >= minBucketCapacity, "INVALID_AMOUNT_TO_SELL");
        require(
            // currentRate * maxRateStep >= newRate >= currentRate
            currentBucketRateNumer *
                maxBucketRateIncreaseNumer *
                newRateDenom >=
                newRateNumer *
                    maxBucketRateIncreaseDenom *
                    currentBucketRateDenom &&
                newRateNumer * currentBucketRateDenom >=
                newRateDenom * currentBucketRateNumer,
            "INVALID_RATE"
        );
        currentBucketStartTimestamp = startTimestamp;
        currentBucketEndTimestamp = endTimestamp;
        currentBucketCapacity = capacity;
        currentBucketRateNumer = newRateNumer;
        currentBucketRateDenom = newRateDenom;
        currentBucketSoldAmount = 0;

        currentBucketId += 1;
        isBucketActive = true;
        emit BucketCreated(
            currentBucketId,
            currentBucketStartTimestamp,
            currentBucketEndTimestamp,
            currentBucketCapacity,
            currentBucketRateNumer,
            currentBucketRateDenom
        );
    }

    function concludeCurrentBucket() public onlyOwner {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(isBucketActive, "BUCKET_NOT_ACTIVE");
        _concludeCurrentBucket();
    }

    function getTotalTokensCollected(
        address token
    ) public view returns (uint256) {
        return totalTokensCollected[token];
    }

    function concludeWholePhase() public onlyOwner {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(!isBucketActive, "BUCKET_ACTIVE");
        isPhaseActive = false;
        emit PhaseConcluded(totalSoldAmount);
    }

    function withdraw(
        address tokenAddress,
        address toAddress,
        uint256 amount
    ) public onlyOwner {
        require(!isBucketActive, "BUCKET_ACTIVE");
        require(isTokenAccepted[tokenAddress], "TOKEN_NOT_ACCEPTED");
        IERC20(tokenAddress).transfer(toAddress, amount);
        emit WithdrawExecuted(tokenAddress, toAddress, amount);
    }

    function getETHPrice() public view returns (uint256) {
        require(wethAddress != address(0), "TOKEN_NOT_ACCEPTED");
        return
            estimateAmountOut(
                wethAddress,
                referenceTokenAddress,
                3000,
                uint128(tenToTheWethTokenDecimals),
                3600
            );
    }

    function getERC20Price(address token) public view returns (uint256) {
        require(isTokenAccepted[token], "TOKEN_NOT_ACCEPTED");
        return
            estimateAmountOut(
                token,
                referenceTokenAddress,
                3000,
                uint128(10 ** IERC20Metadata(token).decimals()),
                3600
            );
    }

    function buyWithETH() public payable {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(isBucketActive, "BUCKET_NOT_ACTIVE");
        require(wethAddress != address(0), "TOKEN_NOT_ACCEPTED");
        require(msg.value > 0, "INVALID_AMOUNT");
        if (block.timestamp >= currentBucketEndTimestamp) {
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

            emit BuyExecuted(currentBucketId, currentBucketSoldAmount);
        }
    }

    function buyWithERC20(address token, uint256 amount) public {
        require(isPhaseActive, "PHASE_NOT_ACTIVE");
        require(isTokenAccepted[token], "TOKEN_NOT_ACCEPTED");
        require(isBucketActive, "BUCKET_NOT_ACTIVE");
        require(amount > 0, "INVALID_AMOUNT");
        if (block.timestamp >= currentBucketEndTimestamp) {
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

            emit BuyExecuted(currentBucketId, currentBucketSoldAmount);
        }
    }

    function estimateAmountOut(
        address tokenIn,
        address tokenOut,
        uint24 fee,
        uint128 amountIn,
        uint24 secondsAgo
    ) internal view returns (uint256) {
        if (tokenIn == tokenOut) return amountIn;

        address pool = IUniswapV3Factory(uniswapV3FactoryAddress).getPool(
            tokenIn,
            tokenOut,
            fee
        );
        int24 tick = SimpleOracleLibrary.consult(pool, secondsAgo);
        return
            SimpleOracleLibrary.getQuoteAtTick(
                tick,
                amountIn,
                tokenIn,
                tokenOut
            );
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
        uint256 tokensToMint = (amount * price * currentBucketRateDenom) /
            tenToTheReferenceTokenDecimals /
            currentBucketRateNumer;
        reducedTokensToMint = min(
            tokensToMint,
            currentBucketCapacity - currentBucketSoldAmount
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
        currentBucketSoldAmount += tokensToMint;
        totalSoldAmount += tokensToMint;
        totalTokensCollected[token] += amount;
        IERC20Mintable(soldTokenAddress).mint(buyer, tokensToMint);
    }

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    function _concludeCurrentBucket() internal {
        isBucketActive = false;
        emit BucketConcluded(currentBucketId, currentBucketSoldAmount);
    }
}
