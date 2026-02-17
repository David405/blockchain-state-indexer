use std::collections::HashMap;

pub struct BalanceState {
    pub balances: HashMap<String, u64>,
}

impl BalanceState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }
    
    pub fn apply_transfer(&mut self, from: &str, to: &str, amount: u64) {
        let from_balance = self.balances.entry(from.to_string()).or_insert(0);
        *from_balance = from_balance.saturating_sub(amount);

        let to_balance = self.balances.entry(to.to_string()).or_insert(0);
        *to_balance += amount;
    }

    pub fn get_balances(&self) -> &HashMap<String, u64> {
        &self.balances
    }
}