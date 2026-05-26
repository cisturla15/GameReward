# GameReward PH

A blockchain-based esports reward system for internet cafe tournaments in the Philippines.

## Problem
Players in local esports cafés in Quezon City often don’t receive fair or verifiable rewards after matches.

## Solution
Rewards are issued as on-chain tokens using Soroban smart contracts on Stellar.

## Timeline
- Day 1: Contract setup
- Day 2: Reward logic
- Day 3: Testing + demo

## Stellar Features
- Soroban smart contracts
- Custom reward asset
- XLM/USDC settlement

## Prerequisites
- Rust
- Soroban CLI

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy

## CLI Example
soroban contract invoke \
--id <contract_id> \
-- reward_player \
--player <address> \
--points 50

## License
MIT