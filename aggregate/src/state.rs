use crate::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MutualCreditAccountAggregateState {
    pub balance: u32, // CENTS
    pub min_balance: u32,
    pub account_number: String,
    pub entity_id: String,
    pub reserved_credit: HashMap<String, u32>, // credit_transfer_id -> amount
}

impl MutualCreditAccountAggregateState {
    /// Returns the regular balance minus the sum of transfer holds
    pub fn available_balance(&self) -> u32 {
        self.balance
            .checked_sub(self.reserved_credit.values().sum::<u32>())
            .unwrap_or(0)
    }

    /// Returns the total amount of credit on hold
    pub fn total_reserved(&self) -> u32 {
        self.reserved_credit.values().sum::<u32>()
    }

    /// Releases the credit associated with a credit transfer hold. Has no affect on actual balance, only available
    pub fn release_credit(self, reservation_id: &str) -> Self {
        let mut new_state = self.clone();
        new_state.reserved_credit.remove(reservation_id);

        new_state
    }

    /// Adds a reservation hold for a given credit transfer. Has no affect on actual balance, only available
    pub fn reserve_credit(self, reservation_id: &str, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state
            .reserved_credit
            .insert(reservation_id.to_string(), amount);
        new_state
    }

    /// Commits held credit. Subtracts held credit from balance. Note: A more realistic mutualcrediting
    /// app might emit an overdrawn/overdraft event if the new balance is less than 0. Here we
    /// just floor the balance at 0. Also note that overcommits shouldn't happen because we reject
    /// attempts to hold beyond available credit
    pub fn commit_credit(self, reservation_id: &str) -> Self {
        let mut new_state = self.clone();
        let amount = new_state.reserved_credit.remove(reservation_id).unwrap_or(0);
        new_state.balance = new_state.balance.checked_sub(amount).unwrap_or(0);
        new_state
    }

    /// Withdraws a given amount of credit
    pub fn withdraw(self, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state.balance = new_state.balance.checked_sub(amount).unwrap_or(0);
        new_state
    }

    /// Deposits a given amount of credit. Ceilings at u32::MAX
    pub fn deposit(self, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state.balance = new_state
            .balance
            .checked_add(amount)
            .unwrap_or(new_state.balance);
        new_state
    }
}
