#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String};

#[contracttype]
pub enum DataKey {
    Reward(Address),
    Admin,
}

#[contract]
pub struct GameRewardContract;

#[contractimpl]
impl GameRewardContract {

    // Initialize admin
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // MVP: reward player after match
    pub fn reward_player(env: Env, player: Address, points: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut balance: i128 = env.storage()
            .persistent()
            .get(&DataKey::Reward(player.clone()))
            .unwrap_or(0);

        balance += points;

        env.storage()
            .persistent()
            .set(&DataKey::Reward(player), &balance);
    }

    pub fn get_reward(env: Env, player: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Reward(player))
            .unwrap_or(0)
    }
}