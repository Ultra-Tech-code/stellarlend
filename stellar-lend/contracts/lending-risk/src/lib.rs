#![no_std]

use lending_types::{calculate_health_factor, is_healthy, BPS_DIVISOR};
use soroban_sdk::{Address, Env};

pub struct RiskManager;

impl RiskManager {
    pub fn check_liquidation_eligibility(
        collateral_value: i128,
        debt_value: i128,
        liquidation_threshold_bps: i128,
    ) -> bool {
        let health_factor =
            calculate_health_factor(collateral_value, debt_value, liquidation_threshold_bps);
        !is_healthy(health_factor)
    }

    pub fn calculate_liquidation_bonus(debt_amount: i128, bonus_bps: i128) -> i128 {
        debt_amount * bonus_bps / BPS_DIVISOR
    }

    pub fn calculate_max_liquidatable(debt_value: i128, close_factor_bps: i128) -> i128 {
        debt_value * close_factor_bps / BPS_DIVISOR
    }

    pub fn validate_borrow_capacity(
        collateral_value: i128,
        existing_debt: i128,
        new_borrow: i128,
        collateral_factor_bps: i128,
    ) -> bool {
        let max_borrow = collateral_value * collateral_factor_bps / BPS_DIVISOR;
        existing_debt + new_borrow <= max_borrow
    }

    pub fn calculate_ltv(debt_value: i128, collateral_value: i128) -> i128 {
        if collateral_value == 0 {
            return BPS_DIVISOR;
        }
        debt_value * BPS_DIVISOR / collateral_value
    }

    pub fn check_concentration_risk(
        asset_value: i128,
        total_pool_value: i128,
        max_concentration_bps: i128,
    ) -> bool {
        if total_pool_value == 0 {
            return true;
        }
        let concentration = asset_value * BPS_DIVISOR / total_pool_value;
        concentration <= max_concentration_bps
    }
}

pub struct RiskMetrics {
    pub health_factor: i128,
    pub ltv_ratio: i128,
    pub liquidation_price: i128,
    pub borrow_capacity: i128,
}

impl RiskMetrics {
    pub fn calculate(
        collateral_value: i128,
        debt_value: i128,
        collateral_factor_bps: i128,
        liquidation_threshold_bps: i128,
    ) -> Self {
        let health_factor =
            calculate_health_factor(collateral_value, debt_value, liquidation_threshold_bps);
        let ltv_ratio = RiskManager::calculate_ltv(debt_value, collateral_value);
        let borrow_capacity = collateral_value * collateral_factor_bps / BPS_DIVISOR;

        let liquidation_price = if collateral_value > 0 {
            debt_value * BPS_DIVISOR / (collateral_value * liquidation_threshold_bps / BPS_DIVISOR)
        } else {
            0
        };

        Self {
            health_factor,
            ltv_ratio,
            liquidation_price,
            borrow_capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidation_eligibility() {
        assert!(RiskManager::check_liquidation_eligibility(
            100_000, 90_000, 8_000
        ));
        assert!(!RiskManager::check_liquidation_eligibility(
            100_000, 50_000, 8_000
        ));
    }

    #[test]
    fn test_liquidation_bonus() {
        let bonus = RiskManager::calculate_liquidation_bonus(100_000, 500);
        assert_eq!(bonus, 5_000);
    }

    #[test]
    fn test_ltv_calculation() {
        assert_eq!(RiskManager::calculate_ltv(50_000, 100_000), 5_000);
        assert_eq!(RiskManager::calculate_ltv(80_000, 100_000), 8_000);
    }

    #[test]
    fn test_borrow_capacity() {
        assert!(RiskManager::validate_borrow_capacity(
            100_000, 50_000, 10_000, 7_500
        ));
        assert!(!RiskManager::validate_borrow_capacity(
            100_000, 70_000, 10_000, 7_500
        ));
    }

    #[test]
    fn test_concentration_risk() {
        assert!(RiskManager::check_concentration_risk(
            30_000, 100_000, 5_000
        ));
        assert!(!RiskManager::check_concentration_risk(
            60_000, 100_000, 5_000
        ));
    }

    #[test]
    fn test_risk_metrics() {
        let metrics = RiskMetrics::calculate(100_000, 50_000, 7_500, 8_000);
        assert_eq!(metrics.ltv_ratio, 5_000);
        assert!(metrics.health_factor > 10_000);
        assert_eq!(metrics.borrow_capacity, 75_000);
    }
}
