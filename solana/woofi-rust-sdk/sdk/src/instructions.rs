use anyhow::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

pub fn try_query_instruction(
    wooconfig: Pubkey,
    wooracle_from: Pubkey,
    woopool_from: Pubkey,
    price_update_from: Pubkey,
    wooracle_to: Pubkey,
    woopool_to: Pubkey,
    price_update_to: Pubkey,
    quote_price_update: Pubkey,
    from_amount: u128
) -> Result<Instruction> {
    let ix = Instruction {
        program_id: woofi::id(),
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &woofi::accounts::TryQuery {
                wooconfig,
                wooracle_from,
                woopool_from,
                price_update_from,
                wooracle_to,
                woopool_to,
                price_update_to,
                quote_price_update
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(
            &woofi::instruction::TryQuery {
                from_amount,
            },
        ),
    };

    Ok(ix)
}

pub fn query_instruction(
    wooconfig: Pubkey,
    wooracle_from: Pubkey,
    woopool_from: Pubkey,
    price_update_from: Pubkey,
    wooracle_to: Pubkey,
    woopool_to: Pubkey,
    price_update_to: Pubkey,
    quote_price_update: Pubkey,
    token_vault_from: Pubkey,
    token_vault_to: Pubkey,
    woopool_quote: Pubkey,
    quote_token_vault: Pubkey,
    from_amount: u128,
    min_to_amount: u128,
) -> Result<Instruction> {
    let ix = Instruction {
        program_id: woofi::id(),
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &woofi::accounts::Query {
                wooconfig,
                wooracle_from,
                woopool_from,
                price_update_from,
                wooracle_to,
                woopool_to,
                price_update_to,
                quote_price_update,
                token_vault_from,
                token_vault_to,
                woopool_quote,
                quote_token_vault
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(
            &woofi::instruction::Query {
                from_amount,
                min_to_amount,
            },
        ),
    };

    Ok(ix)
}

pub fn swap_instruction(
    payer: Pubkey,
    wooconfig: Pubkey,
    wooracle_from: Pubkey,
    woopool_from: Pubkey,
    price_update_from: Pubkey,
    wooracle_to: Pubkey,
    woopool_to: Pubkey,
    price_update_to: Pubkey,
    quote_price_update: Pubkey,
    token_vault_from: Pubkey,
    token_vault_to: Pubkey,
    woopool_quote: Pubkey,
    quote_token_vault: Pubkey,
    token_owner_account_from: Pubkey,
    token_owner_account_to: Pubkey,
    rebate_to: Pubkey,
    from_amount: u128,
    min_to_amount: u128,
) -> Result<Instruction> {
    let ix = Instruction {
        program_id: woofi::id(),
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &woofi::accounts::Swap {
                payer,
                wooconfig,
                wooracle_from,
                woopool_from,
                price_update_from,
                wooracle_to,
                woopool_to,
                price_update_to,
                quote_price_update,
                token_vault_from,
                token_vault_to,
                woopool_quote,
                quote_token_vault,
                token_program: spl_token::id(),
                token_owner_account_from,
                token_owner_account_to,
                rebate_to
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(
            &woofi::instruction::Swap {
                from_amount,
                min_to_amount,
            },
        ),
    };

    Ok(ix)
}
