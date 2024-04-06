use std::{
    io::{stdout, Write},
    time::Duration,
};

use solana_client::{
    client_error::{ClientError, ClientErrorKind, Result as ClientResult},
    rpc_config::RpcSendTransactionConfig,
};
use solana_program::instruction::Instruction;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    signature::{Signature, Signer},
    transaction::Transaction,
};
use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};

use crate::Miner;

const RPC_RETRIES: usize = 1;
const GATEWAY_RETRIES: usize = 4;
const CONFIRM_RETRIES: usize = 4;

impl Miner {
    pub async fn send_and_confirm(
        &self,
        ixs: &[Instruction],
        skip_confirm: bool,
    ) -> ClientResult<Signature> {
        let client = &self.client;
        let exec_client = &self.exec_client;
        let mut stdout = stdout();
        let signer = self.signer();

        // Return error if balance is zero
        let balance = client
            .get_balance_with_commitment(&signer.pubkey(), CommitmentConfig::confirmed())
            .await
            .unwrap();
        if balance.value <= 0 {
            return Err(ClientError {
                request: None,
                kind: ClientErrorKind::Custom("Insufficient SOL balance".into()),
            });
        }

        // Build tx
        let (mut hash, mut slot) = client
            .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
            .await
            .unwrap();
        let mut send_cfg = RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Confirmed),
            encoding: Some(UiTransactionEncoding::Base64),
            max_retries: Some(RPC_RETRIES),
            min_context_slot: Some(slot),
        };
        let mut tx = Transaction::new_with_payer(ixs, Some(&signer.pubkey()));
        tx.sign(&[&signer], hash);

        // Submit tx
        let sig = exec_client.send_transaction_with_config(&tx, send_cfg)?;
        return Ok(sig);
    }
}
