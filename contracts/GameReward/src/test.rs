#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_happy_path() {
    let env = Env::default();
    let contract = GameRewardContract;
    let admin = Address::generate(&env);
    let player = Address::generate(&env);

    contract.init(env.clone(), admin.clone());
    contract.reward_player(env.clone(), player.clone(), 50);

    assert_eq!(contract.get_reward(env.clone(), player.clone()), 50);
}

#[test]
fn test_unauthorized_fails() {
    let env = Env::default();
    let contract = GameRewardContract;

    let admin = Address::generate(&env);
    let player = Address::generate(&env);

    contract.init(env.clone(), admin.clone());

    // no auth simulation (would fail in real env)
    let result = std::panic::catch_unwind(|| {
        contract.reward_player(env.clone(), player.clone(), 50);
    });

    assert!(result.is_err());
}

#[test]
fn test_state_update() {
    let env = Env::default();
    let contract = GameRewardContract;
    let admin = Address::generate(&env);
    let player = Address::generate(&env);

    contract.init(env.clone(), admin.clone());
    contract.reward_player(env.clone(), player.clone(), 30);
    contract.reward_player(env.clone(), player.clone(), 20);

    assert_eq!(contract.get_reward(env.clone(), player.clone()), 50);
}

#[test]
fn test_zero_initial_state() {
    let env = Env::default();
    let contract = GameRewardContract;
    let player = Address::generate(&env);

    assert_eq!(contract.get_reward(env.clone(), player.clone()), 0);
}

#[test]
fn test_multiple_players() {
    let env = Env::default();
    let contract = GameRewardContract;
    let admin = Address::generate(&env);

    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);

    contract.init(env.clone(), admin.clone());
    contract.reward_player(env.clone(), p1.clone(), 10);
    contract.reward_player(env.clone(), p2.clone(), 25);

    assert_eq!(contract.get_reward(env.clone(), p1.clone()), 10);
    assert_eq!(contract.get_reward(env.clone(), p2.clone()), 25);
}