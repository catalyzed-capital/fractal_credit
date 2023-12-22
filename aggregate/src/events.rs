use crate::*;

impl From<AccountCreated> for MutualCreditAccountAggregateState {
    fn from(input: AccountCreated) -> MutualCreditAccountAggregateState {
        MutualCreditAccountAggregateState {
            balance: input.initial_balance.unwrap_or(0) as _,
            min_balance: input.min_balance.unwrap_or(0) as _,
            account_number: input.account_number,
            entity_id: input.entity_id,
            reserved_credit: HashMap::new(),
        }
    }
}

pub(crate) fn apply_account_created(input: AccountCreated) -> Result<StateAck> {
    Ok(StateAck::ok(Some(MutualCreditAccountAggregateState::from(input))))
}

pub(crate) fn apply_credit_deposited(
    input: CreditDeposited,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting credit deposited event. Account {} does not exist.",
            input.account_number
        );
        return Ok(StateAck::error(
            "Account does not exist",
            None::<MutualCreditAccountAggregateState>,
        ));
    };
    let state = MutualCreditAccountAggregateState {
        balance: state.balance + input.amount as u32,
        ..state
    };
    Ok(StateAck::ok(Some(state)))
}

pub(crate) fn apply_credit_released(
    input: CreditReleased,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting credit released event. Account {} does not exist.",
            input.account_number
        );
        return Ok(StateAck::error(
            "Account does not exist",
            None::<MutualCreditAccountAggregateState>,
        ));
    };
    let state = state.release_credit(&input.credit_transfer_id);
    Ok(StateAck::ok(Some(state)))
}

pub(crate) fn apply_credit_committed(
    input: CreditCommitted,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting credit committed event. Account {} does not exist.",
            input.account_number
        );
        return Ok(StateAck::error(
            "Account does not exist",
            None::<MutualCreditAccountAggregateState>,
        ));
    };
    let state = state.commit_credit(&input.credit_transfer_id);
    Ok(StateAck::ok(Some(state)))
}

pub(crate) fn apply_credit_reserved(
    input: CreditReserved,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting credit reserved event. Account {} does not exist.",
            input.account_number
        );
        return Ok(StateAck::error(
            "Account does not exist",
            None::<MutualCreditAccountAggregateState>,
        ));
    };
    let state = state.reserve_credit(&input.credit_transfer_id, input.amount as u32);
    Ok(StateAck::ok(Some(state)))
}

pub(crate) fn apply_credit_withdrawn(
    input: CreditWithdrawn,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting credit withdrawn event. Account {} does not exist.",
            input.account_number
        );
        return Ok(StateAck::error(
            "Account does not exist",
            None::<MutualCreditAccountAggregateState>,
        ));
    };
    let state = state.withdraw(input.amount as u32);
    Ok(StateAck::ok(Some(state)))
}

pub(crate) fn apply_credit_transfer_initiated(
    _input: CreditTransferInitiated,
    state: Option<MutualCreditAccountAggregateState>,
) -> Result<StateAck> {
    // We don't currently change internal state because of this. The first time a credit transfer
    // impacts the the account is when credit are reserved (by the process manager)
    Ok(StateAck::ok(state))
}
