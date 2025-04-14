use anchor_spl::associated_token::get_associated_token_address;
use anyhow::{format_err, Context, Result};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use rpc::{get_instruction_logs, send_txn, try_from_program_logs};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, signature::{Keypair, Signer}, transaction::Transaction
};
use woofi_sdk::{constants::ONE_E9_U128, util::{SOL, USDC}};
use woofi_sdk::{util::{get_wooammpool_address, get_wooconfig_address, get_woofi_accounts, QueryResult, SupportedTokens}, woofi_sdk::swap};
use woofi_sdk::woofi_sdk::try_query;
use std::env;

use jupiter_amm_interface::{AccountMap, Amm, AmmContext, ClockRef, KeyedAccount, QuoteParams, SwapAndAccountMetas, SwapMode, SwapParams};

use solana_sdk::{clock::Clock, sysvar};
use woofi_sdk::{jup::WoofiSwap};


mod rpc;

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    http_url: String,
    payer_path: String,
}

fn load_cfg(opts: &Opts) -> Result<ClientConfig, Box<dyn std::error::Error>> {
    dotenv().ok();

    let http_url = opts
        .http_url
        .clone()
        .unwrap_or_else(||
            env::var("HTTP_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string())
        );
    let payer_path = opts
        .payer_path
        .clone()
        .unwrap_or_else(||
            env::var("PAYER_PATH")
            .unwrap_or_else(|_| {
                let mut ret = env::var("HOME").unwrap();
                ret.push_str("/.config/solana/id.json");
                return ret;
            })
        );

    Ok(ClientConfig {
        http_url,
        payer_path,
    })
}

fn read_keypair_file(s: &str) -> Result<Keypair> {
    solana_sdk::signature::read_keypair_file(s)
        .map_err(|_| format_err!("failed to read keypair from {}", s))
}
#[derive(Parser, Debug)]
#[clap(name = "woofi-cli")]
pub struct Opts {
    #[clap(long, env = "HTTP_URL")]
    http_url: Option<String>,

    #[clap(long, env = "PAYER_PATH")]
    payer_path: Option<String>,

    #[clap(subcommand)]
    command: WoofiCommands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum WoofiCommands {
    TryQuery {
        from_token: SupportedTokens,
        to_token: SupportedTokens,
        from_amount: u128,
    },
    Swap {
        from_token: SupportedTokens,
        to_token: SupportedTokens,
        from_amount: u128,
        min_to_amount: u128,
    },
    JupQuery {
        from_token: SupportedTokens,
        to_token: SupportedTokens,
        from_amount: u128,  
    },
    JupSwap {
        from_token: SupportedTokens,
        to_token: SupportedTokens,
        from_amount: u128,
        min_to_amount: u128,
    },
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let pool_config = load_cfg(&opts).unwrap();
    // cluster params.
    let payer = read_keypair_file(&pool_config.payer_path)?;
    println!("pubkey: {}", payer.pubkey());
    
    // solana rpc client
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());

    let opts = Opts::parse();
    match opts.command {
        WoofiCommands::TryQuery {
            from_token,
            to_token,
            from_amount,
        } => {
            let ix = try_query(from_token, to_token, from_amount)?;

            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &vec![ix],
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );
        
            let logs = get_instruction_logs(&rpc_client, &txn).unwrap();
            logs.iter().for_each(|x| println!("{}", x));
        
            let result: QueryResult = try_from_program_logs(logs, &woofi::id())?;
        
            println!("to_amount:{}", result.to_amount);
            println!("swap_fee:{}", result.swap_fee);
        }
        WoofiCommands::Swap {
            from_token,
            to_token,
            from_amount,
            min_to_amount,
        } => {
            let from_token_accounts = get_woofi_accounts(&from_token);
            let to_token_accounts = get_woofi_accounts(&to_token);

            let from_account = get_associated_token_address(&payer.pubkey(), &from_token_accounts.token_mint);
            let to_account = get_associated_token_address(&payer.pubkey(), &to_token_accounts.token_mint);
            let ix = swap(
                &rpc_client,
                payer.pubkey(),
                from_token,
                to_token,
                from_account,
                to_account,
                payer.pubkey(),
                from_amount,
                min_to_amount)?;
            
            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &vec![ix],
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );

            let signature = send_txn(&rpc_client, &txn, true)?;
            println!("transaction sent, signature:{}", signature);
            println!("please view on explorer");

            let logs = get_instruction_logs(&rpc_client, &txn).unwrap();
            logs.iter().for_each(|x| println!("{}", x));
        }
        WoofiCommands::JupQuery {
            from_token,
            to_token,
            from_amount,
        } => {
            let from_token_accounts = get_woofi_accounts(&from_token);
            let to_token_accounts = get_woofi_accounts(&to_token);

            let mut woofi_swap = create_woofi_swap(&rpc_client)?;
            let pubkeys = woofi_swap.get_accounts_to_update();
            let accounts_map: AccountMap = pubkeys
                .iter()
                .zip(rpc_client.get_multiple_accounts(&pubkeys)?)
                .map(|(key, acc)| (*key, acc.unwrap()))
                .collect();
        
            woofi_swap.update(&accounts_map)?;
        
            let result = woofi_swap.quote(&QuoteParams{
                amount: from_amount as u64,
                input_mint: from_token_accounts.token_mint,
                output_mint: to_token_accounts.token_mint,
                swap_mode: SwapMode::ExactIn
            })?;

            let famount = from_amount as f64;
            println!("Getting quote for selling {} SOL", famount / ONE_E9_U128 as f64);
            println!("result.out_amount:{}", result.out_amount);
            println!("result.in_amount:{}", result.in_amount);
            println!("result.fee_amount:{}", result.fee_amount);
            println!("result.fee_mint:{}", result.fee_mint);
        }
        WoofiCommands::JupSwap {
            from_token,
            to_token,
            from_amount,
            min_to_amount,
        } => {
            let from_token_accounts = get_woofi_accounts(&from_token);
            let to_token_accounts = get_woofi_accounts(&to_token);

            let from_account = get_associated_token_address(&payer.pubkey(), &from_token_accounts.token_mint);
            let to_account = get_associated_token_address(&payer.pubkey(), &to_token_accounts.token_mint);

            let mut woofi_swap = create_woofi_swap(&rpc_client)?;
            
            let swap_params = SwapParams {
                source_mint: from_token_accounts.token_mint,
                destination_mint: to_token_accounts.token_mint,
                source_token_account: from_account,
                destination_token_account: to_account,
                token_transfer_authority: payer.pubkey(),
                open_order_address: None,
                quote_mint_to_referrer: None,
                in_amount: from_amount as u64,
                out_amount: 0,
                jupiter_program_id: &woofi::id(),
                missing_dynamic_accounts_as_default: false,
            };
            let SwapAndAccountMetas {
                swap,
                account_metas,
            } = woofi_swap.get_swap_and_account_metas(&swap_params).unwrap();

            let ix = Instruction {
                program_id: woofi::id(),
                accounts: account_metas,
                data: anchor_lang::InstructionData::data(
                    &woofi::instruction::Swap {
                        from_amount,
                        min_to_amount,
                    },
                ),
            };
            
            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &vec![ix],
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );

            let signature = send_txn(&rpc_client, &txn, true)?;
            println!("transaction sent, signature:{}", signature);
            println!("please view on explorer");

            let logs = get_instruction_logs(&rpc_client, &txn).unwrap();
            logs.iter().for_each(|x| println!("{}", x));
        }
    }
    Ok(())
}

pub fn create_woofi_swap(rpc_client: &RpcClient) -> anyhow::Result<WoofiSwap> {
    let wooconfig = get_wooconfig_address(&woofi::id()).0;
    // TODO Prince: currently only support SOL,USDC pair, more later
    let wooammpool = get_wooammpool_address(
                              &wooconfig,
                              &SOL,
                              &USDC,
                              &woofi::id()
                          ).0;

    let account = rpc_client.get_account(&wooammpool)?;

    let amm_context = get_amm_context(&rpc_client)?;

    let market_account = KeyedAccount {
        key: woofi_sdk::id(),
        account,
        params: None,
    };

    let woofi_swap = WoofiSwap::from_keyed_account(&market_account, &amm_context).unwrap();

    println!("ProgramId:{}", woofi_swap.program_id);

    Ok(woofi_swap)
}

pub fn get_clock(rpc_client: &RpcClient) -> anyhow::Result<Clock> {
    let clock_data = rpc_client
        .get_account_with_commitment(&sysvar::clock::ID, rpc_client.commitment())?
        .value
        .context("Failed to get clock account")?;

    let clock: Clock = bincode::deserialize(&clock_data.data)
        .context("Failed to deserialize sysvar::clock::ID")?;

    Ok(clock)
}

pub fn get_clock_ref(rpc_client: &RpcClient) -> anyhow::Result<ClockRef> {
    let clock = get_clock(rpc_client)?;
    Ok(ClockRef::from(clock))
}

pub fn get_amm_context(rpc_client: &RpcClient) -> anyhow::Result<AmmContext> {
    Ok(AmmContext {
        clock_ref: get_clock_ref(rpc_client)?,
    })
}