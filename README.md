# arbitrage-bot-bellman-ford

Educational project for liquidity pool arbitraging using parallel graph algorithm

## Intro

Detecting arbitraging cross liquidity pools is an important activity to make pool prices converge to market price.

This activity needs pools monitoring and could be conceptualized as shortest circuit detection in an oriented graph representing asset as nodes and pool exchange rate as edges.

## Example

Let's consider a set of pools in a group of DEX : 

DEX 1 : wbtc/eth
DEX 2 : eth/usdc
DEX 3 : usdc/wbtc

## Modules description

### Dex scrapper

To build the initial Graph, we need to scrapp some pool ..... to be continued
