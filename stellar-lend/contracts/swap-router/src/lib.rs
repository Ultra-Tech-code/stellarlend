#![no_std]

pub mod integrations;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AMMProtocol {
    Phoenix = 0,
    Aquarius = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapRoute {
    pub protocol: AMMProtocol,
    pub pool_address: Address,
    pub asset_in: Address,
    pub asset_out: Address,
}

#[contract]
pub struct SwapRouterContract;

#[contractimpl]
impl SwapRouterContract {
    pub fn swap_exact_in(
        env: Env,
        caller: Address,
        amount_in: i128,
        min_amount_out: i128,
        routes: Vec<SwapRoute>,
    ) -> Result<i128, &'static str> {
        caller.require_auth();

        if routes.is_empty() {
            return Err("No swap routes provided");
        }

        let mut current_amount = amount_in;

        for route in routes.iter() {
            current_amount = match route.protocol {
                AMMProtocol::Phoenix => {
                    integrations::phoenix::swap(&env, &route.pool_address, &route.asset_in, &route.asset_out, current_amount)?
                }
                AMMProtocol::Aquarius => {
                    integrations::aquarius::swap(&env, &route.pool_address, &route.asset_in, &route.asset_out, current_amount)?
                }
            };
        }

        if current_amount < min_amount_out {
            return Err("Slippage tolerance exceeded");
        }

        Ok(current_amount)
    }
}
