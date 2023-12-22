use serde::{Deserialize, Serialize};

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "process_manager",
    entity: "credit transfer"
});

#[async_trait]
impl CreditTransferProcessManager for CreditTransferProcessManagerImpl {
    async fn handle_credit_released(
        &self,
        _input: CreditReleased,
        _state: Option<CreditTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        // release of credit is the termination of a transfer process
        Ok(ProcessManagerAck::ok(
            None::<CreditTransferProcessManagerState>,
            vec![],
        ))
    }
    
    async fn handle_credit_committed(
        &self,
        _input: CreditCommitted,
        _state: Option<CreditTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        // commitment of credit is the termination of a transfer process
        Ok(ProcessManagerAck::ok(
            None::<CreditTransferProcessManagerState>,
            vec![],
        ))
    }

    async fn handle_credit_reserved(
        &self,
        _input: CreditReserved,
        state: Option<CreditTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let Some(mut state) = state else {
            return Ok(ProcessManagerAck::ok(
                None::<CreditTransferProcessManagerState>,
                vec![],
            ));
        };
        state.status = TransferStatus::CreditReserved;
        Ok(ProcessManagerAck::ok(Some(state), vec![]))
    }

    async fn handle_credit_transfer_succeeded(
        &self,
        input: CreditTransferSucceeded,
        state: Option<CreditTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let Some(mut state) = state else {
            return Ok(ProcessManagerAck::ok(
                None::<CreditTransferProcessManagerState>,
                vec![],
            ));
        };
        state.status = TransferStatus::TransferCompleted;
        let cmd = CommitCredit {
            account_number: state.account_number.to_string(),
            entity_id: state.entity_id.to_string(),
            credit_transfer_id: input.credit_transfer_id.to_string(),
        };

        Ok(ProcessManagerAck::ok(
            Some(state),
            vec![OutputCommand::new(
                CommitCredit::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
        ))
    }

    async fn handle_credit_transfer_initiated(
        &self,
        input: CreditTransferInitiated,
        _state: Option<CreditTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let state = CreditTransferProcessManagerState::new(&input);

        let cmd = ReserveCredit {
            entity_id: input.entity_id,
            account_number: input.account_number,
            amount: input.amount,
            credit_transfer_id: input.credit_transfer_id.to_string(),
        };

        Ok(ProcessManagerAck::ok(
            Some(state),
            vec![OutputCommand::new(
                ReserveCredit::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
        ))
    }

    async fn handle_credit_transfer_failed(
        &self,
        input: CreditTransferFailed,
        state: Option<CreditTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let Some(state) = state else {
            return Ok(ProcessManagerAck::ok(
                None::<CreditTransferProcessManagerState>,
                vec![],
            ));
        };
        let cmd = ReleaseCredit {
            account_number: state.account_number.to_string(),
            entity_id: state.entity_id.to_string(),
            credit_transfer_id: input.credit_transfer_id.to_string(),
        };
        Ok(ProcessManagerAck::ok(
            Some(state),
            vec![OutputCommand::new(
                ReleaseCredit::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditTransferProcessManagerState {
    pub credit_transfer_id: String,
    pub account_number: String,
    pub entity_id: String,
    pub amount: u32,
    pub target_routing_number: String,
    pub target_account_number: String,
    pub status: TransferStatus,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum TransferStatus {
    Requested,
    CreditReserved,
    TransferInitiated,
    TransferCompleted,
    TransferFailed,
    #[default]
    Unknown,
}

impl CreditTransferProcessManagerState {
    pub fn to_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap_or_default()
    }
}

impl CreditTransferProcessManagerState {
    pub fn new(event: &CreditTransferInitiated) -> CreditTransferProcessManagerState {
        let event = event.clone();
        CreditTransferProcessManagerState {
            credit_transfer_id: event.credit_transfer_id,
            account_number: event.account_number,
            entity_id: event.entity_id,
            amount: event.amount as u32,
            target_routing_number: event.target_routing_number,
            target_account_number: event.target_account_number,
            status: TransferStatus::Requested,
        }
    }
}

const STREAM: &str = "mutualcreditaccount";
