use crate::*;

pub(crate) fn handle_reserve_credit(
    input: ReserveCredit,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<EventList> {
    let Some(old_state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to reserve credit. Account {} does not exist.",
            input.account_number
        ));
    };
    let avail_balance = old_state.available_balance();
    if input.amount as u32 > avail_balance {
        error!(
            "Rejecting command to reserve credit, account {} does not have sufficient credit. Available {}",
            &input.account_number, avail_balance
        );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            CreditReserved::TYPE,
            STREAM,
            &CreditReserved {
                account_number: input.account_number.to_string(),
                credit_transfer_id: input.credit_transfer_id,
                entity_id: old_state.entity_id.to_string(),
                amount: input.amount,
            },
        )])
    }
}

pub(crate) fn handle_release_credit(
    input: ReleaseCredit,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<EventList> {
    let Some(old_state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to release credit. Account {} does not exist.",
            input.account_number
        ));
    };

    if old_state
        .reserved_credit
        .contains_key(&input.credit_transfer_id)
    {
        Ok(vec![Event::new(
            CreditReleased::TYPE,
            STREAM,
            &CreditReleased {
                entity_id: input.entity_id,
                account_number: input.account_number.to_string(),
                credit_transfer_id: input.credit_transfer_id.to_string(),
            },
        )])
    } else {
        error!(
                "Rejecting command to release credit, account {} does not have a credit transfer hold for {}",
                &input.account_number, input.credit_transfer_id
            );
        Ok(vec![])
    }
}

pub(crate) fn handle_commit_credit(
    input: CommitCredit,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<EventList> {
    let Some(old_state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to commit credit. Account {} does not exist.",
            input.account_number
        ));
    };

    if old_state
        .reserved_credit
        .contains_key(&input.credit_transfer_id)
    {
        Ok(vec![Event::new(
            CreditCommitted::TYPE,
            STREAM,
            &CreditCommitted {
                entity_id: input.entity_id,
                account_number: input.account_number.to_string(),
                credit_transfer_id: input.credit_transfer_id.to_string(),
            },
        )])
    } else {
        error!(
                "Rejecting command to commit credit, account {} does not have a credit transfer hold for {}",
                &input.account_number, input.credit_transfer_id
            );
        Ok(vec![])
    }
}

pub(crate) fn handle_create_account(input: CreateAccount) -> Result<EventList> {
    Ok(vec![Event::new(
        AccountCreated::TYPE,
        STREAM,
        &AccountCreated {
            initial_balance: input.initial_balance,
            account_number: input.account_number.to_string(),
            min_balance: input.min_balance,
            entity_id: input.entity_id,
        },
    )])
}

pub(crate) fn handle_withdraw_credit(
    input: WithdrawCredit,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<EventList> {
    let Some(state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to withdraw credit. Account {} does not exist.",
            input.account_number
        ));
    };

    if state.available_balance() < input.amount as u32 {
        error!(
                "Rejecting command to withdraw credit, account {} does not have sufficient credit. Available {}",
                &input.account_number, state.available_balance()
            );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            CreditWithdrawn::TYPE,
            STREAM,
            &CreditWithdrawn {
                note: input.note,
                account_number: input.account_number.to_string(),
                amount: input.amount,
                entity_id: input.entity_id,
            },
        )])
    }
}

pub(crate) fn handle_transfer_credit(
    input: TransferCredit,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<EventList> {
    let Some(state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to credit credit. Account {} does not exist.",
            input.account_number
        ));
    };

    if state.available_balance() < input.amount as u32 {
        error!(
                "Rejecting command to credit credit, account {} does not have sufficient credit. Available {}",
                &input.account_number, state.available_balance()
            );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            CreditTransferInitiated::TYPE,
            STREAM,
            &CreditTransferInitiated {
                note: input.note,
                account_number: input.target_account_number,
                target_routing_number: input.target_routing_number,
                target_account_number: input.account_number,
                amount: input.amount,
                entity_id: input.entity_id,
                credit_transfer_id: input.credit_transaction_id,
            },
        )])
    }
}

pub(crate) fn handle_deposit_credit(
    input: DepositCredit,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<EventList> {
    if state.is_none() {
        return Err(anyhow::anyhow!(
            "Rejected command to deposit credit. Account {} does not exist.",
            input.account_number
        ));
    };

    Ok(vec![Event::new(
        CreditDeposited::TYPE,
        STREAM,
        &CreditDeposited {
            note: input.note,
            account_number: input.account_number.to_string(),
            amount: input.amount,
            entity_id: input.entity_id,
        },
    )])
}
