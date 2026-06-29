use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

pub const EVENT_VERSION: u32 = 1;

/// Emits a standardized event across the protocol.
/// Naming convention for `action`: {Module}_{Action} (e.g., Pool_Deposited)
/// Standard indexed fields: caller, asset, amount
pub fn emit_protocol_event(
    env: &Env,
    module: &str,
    action: &str,
    caller: Address,
    asset: Address,
    amount: i128,
) {
    let mut module_action = String::from_str(env, module);
    module_action.append(&String::from_str(env, "_"));
    module_action.append(&String::from_str(env, action));

    let topics = (
        Symbol::new(env, "PROTOCOL_EVENT"),
        module_action,
        caller,
        asset,
    );

    let data = (amount, EVENT_VERSION);

    env.events().publish(topics, data);
}

#[macro_export]
macro_rules! emit_event {
    ($env:expr, $module:expr, $action:expr, $caller:expr, $asset:expr, $amount:expr) => {
        $crate::events::emit_protocol_event($env, $module, $action, $caller, $asset, $amount)
    };
}
