use anyhow::Result;
use serde::{Deserialize, Serialize};

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "projector",
    entity: "mutual credit account"
});

mod store;

#[async_trait]
impl MutualCreditAccountProjector for MutualCreditAccountProjectorImpl {
    async fn handle_account_created(&self, input: AccountCreated) -> Result<()> {
        store::initialize_account(input).await
    }

    async fn handle_credit_deposited(&self, input: CreditDeposited) -> Result<()> {
        store::record_credit_deposited(input).await
    }

    async fn handle_credit_reserved(&self, input: CreditReserved) -> Result<()> {
        store::record_credit_reserved(input).await
    }

    async fn handle_credit_withdrawn(&self, input: CreditWithdrawn) -> Result<()> {
        store::record_credit_withdrawn(input).await
    }

    async fn handle_credit_released(&self, input: CreditReleased) -> Result<()> {
        store::record_credit_released(input).await
    }

    async fn handle_credit_transfer_initiated(&self, _input: CreditTransferInitiated) -> Result<()> {
        Ok(())
    }
}

