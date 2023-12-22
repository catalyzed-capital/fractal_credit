use anyhow::Result;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::error;

mod commands;
mod events;
mod state;

use state::MutualCreditAccountAggregateState;

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "aggregate",
    entity: "mutual credit account"
});

impl MutualCreditAccountAggregate for MutualCreditAccountAggregateImpl {
    // -- Commands --
    fn handle_reserve_credit(
        &self,
        input: ReserveCredit,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_reserve_credit(input, state)
    }

    fn handle_release_credit(
        &self,
        input: ReleaseCredit,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_release_credit(input, state)
    }

    fn handle_commit_credit(
        &self,
        input: CommitCredit,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_commit_credit(input, state)
    }

    fn handle_create_account(
        &self,
        input: CreateAccount,
        _state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_create_account(input)
    }

    fn handle_withdraw_credit(
        &self,
        input: WithdrawCredit,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_withdraw_credit(input, state)
    }

    fn handle_transfer_credit(
        &self,
        input: TransferCredit,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_transfer_credit(input, state)
    }

    fn handle_deposit_credit(
        &self,
        input: DepositCredit,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_deposit_credit(input, state)
    }

    // -- Events --

    fn apply_account_created(
        &self,
        input: AccountCreated,
        _state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_account_created(input)
    }

    fn apply_credit_deposited(
        &self,
        input: CreditDeposited,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_credit_deposited(input, state)
    }

    fn apply_credit_released(
        &self,
        input: CreditReleased,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_credit_released(input, state)
    }

    fn apply_credit_committed(
        &self,
        input: CreditCommitted,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_credit_committed(input, state)
    }

    fn apply_credit_reserved(
        &self,
        input: CreditReserved,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_credit_reserved(input, state)
    }

    fn apply_credit_withdrawn(
        &self,
        input: CreditWithdrawn,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_credit_withdrawn(input, state)
    }

    fn apply_credit_transfer_initiated(
        &self,
        input: CreditTransferInitiated,
        state: Option<MutualCreditAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_credit_transfer_initiated(input, state)
    }
}

const STREAM: &str = "mutualcreditaccount";
