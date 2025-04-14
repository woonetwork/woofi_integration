use anchor_lang::AnchorDeserialize;
use anyhow::{anyhow, Error, Result};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, program_pack::Pack as TokenPack, pubkey::Pubkey, signature::Signature, transaction::Transaction
};
use std::convert::Into;

pub fn get_instruction_logs(
    client: &RpcClient,
    transaction: &Transaction
) -> Result<Vec<String>> {
    let result = client.simulate_transaction(transaction)?;
    if result.value.err.is_some() {
        return Err(result.value.err.unwrap().into());
    }
    let logs = result.value.logs.ok_or(anyhow!("DeserializationError"))?;
    Ok(logs)
}

pub fn send_txn(client: &RpcClient, txn: &Transaction, wait_confirm: bool) -> Result<Signature> {
    Ok(client.send_and_confirm_transaction_with_spinner_and_config(
        txn,
        if wait_confirm {
            CommitmentConfig::confirmed()
        } else {
            CommitmentConfig::processed()
        },
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        },
    )?)
}

pub fn try_from_program_logs<T: AnchorDeserialize>(
    program_logs: Vec<String>,
    program_id: &Pubkey,
) -> std::result::Result<T, Error> {
    // A Program's return data appears in the log in this format:
    // "Program return: <program-id> <program-generated-data-in-base64>"
    // https://github.com/solana-labs/solana/blob/b8837c04ec3976c9c16d028fbee86f87823fb97f/program-runtime/src/stable_log.rs#L68
    let preimage = format!("Program return: {} ", program_id.to_string());

    // Extract the return data after Program return: <program-id>
    let get_return_data_base64 = program_logs
        .iter()
        .find(|&s| s.starts_with(&preimage))
        .ok_or(anyhow!("AccountDidNotDeserialize"))?
        .strip_prefix(&preimage)
        .ok_or(anyhow!("AccountDidNotDeserialize"))?;

    let decoded = base64::decode(get_return_data_base64)
        .map_err(|_err| anyhow!("AccountDidNotDeserialize"))?;

    T::try_from_slice(&decoded).map_err(|_err| anyhow!("AccountDidNotDeserialize"))
}

pub fn get_token_account<T: TokenPack>(client: &RpcClient, addr: &Pubkey) -> Result<T> {
    let account = client
        .get_account_with_commitment(addr, CommitmentConfig::processed())?
        .value
        .map_or(Err(anyhow!("Account not found")), Ok)?;
    T::unpack_from_slice(&account.data).map_err(Into::into)
}

pub fn get_multiple_accounts(
    client: &RpcClient,
    pubkeys: &[Pubkey],
) -> Result<Vec<Option<Account>>> {
    Ok(client.get_multiple_accounts(pubkeys)?)
}
