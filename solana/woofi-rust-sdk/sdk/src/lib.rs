/*

░██╗░░░░░░░██╗░█████╗░░█████╗░░░░░░░███████╗██╗
░██║░░██╗░░██║██╔══██╗██╔══██╗░░░░░░██╔════╝██║
░╚██╗████╗██╔╝██║░░██║██║░░██║█████╗█████╗░░██║
░░████╔═████║░██║░░██║██║░░██║╚════╝██╔══╝░░██║
░░╚██╔╝░╚██╔╝░╚█████╔╝╚█████╔╝░░░░░░██║░░░░░██║
░░░╚═╝░░░╚═╝░░░╚════╝░░╚════╝░░░░░░░╚═╝░░░░░╚═╝

*
* MIT License
* ===========
*
* Copyright (c) 2020 WooTrade
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
#[cfg(feature = "devnet")]
declare_id!("Es677W33uwrXLSqjV3rqcz5sftyarupdV3vDpQ9LXGow");

#[cfg(not(feature = "devnet"))]
declare_id!("woofiCKJyDKxswhzu98HRW2v52AfjBKHkKtHEzV4ncV");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod util;
pub mod jup;

use anchor_lang::{declare_id, AccountDeserialize};
use anyhow::{anyhow, Result};

use woofi::state::WooPool;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use instructions::{query_instruction, swap_instruction, try_query_instruction};
use util::{get_woofi_accounts, get_woofi_pool_params, QUOTE_TOKEN, SupportedTokens};

pub mod woofi_sdk {

    use crate::*;

    pub fn try_query(
        token_from: SupportedTokens,
        token_to: SupportedTokens,
        from_amount: u128
    ) -> Result<Instruction> {
        if token_from == token_to {
            return Err(anyhow!("From token cannot be same with To token"));
        }

        let token_from_accounts = get_woofi_accounts(&token_from);
        let token_to_accounts = get_woofi_accounts(&token_to);
        let token_from_params = get_woofi_pool_params(&token_from_accounts);
        let token_to_params = get_woofi_pool_params(&token_to_accounts);
        let token_quote_accounts = get_woofi_accounts(&QUOTE_TOKEN);

        try_query_instruction(
            token_from_params.wooconfig,
            token_from_params.wooracle,
            token_from_params.woopool,
            token_from_accounts.price_update,
            token_to_params.wooracle,
            token_to_params.woopool,
            token_to_accounts.price_update,
            token_quote_accounts.price_update,
            from_amount
        )
    }

    pub fn query(
        client: &RpcClient,
        token_from: SupportedTokens,
        token_to: SupportedTokens,
        from_amount: u128,
        min_to_amount: u128
    ) -> Result<Instruction> {
        if token_from == token_to {
            return Err(anyhow!("From token cannot be same with To token"));
        }

        let token_from_accounts = get_woofi_accounts(&token_from);
        let token_to_accounts = get_woofi_accounts(&token_to);
        let token_from_params = get_woofi_pool_params(&token_from_accounts);
        let token_to_params = get_woofi_pool_params(&token_to_accounts);
        let token_quote_accounts = get_woofi_accounts(&QUOTE_TOKEN);
        let token_quote_params = get_woofi_pool_params(&token_quote_accounts);

        let woopool_account_from = client.get_account(&token_from_params.woopool)?;
        let woopool_from = WooPool::try_deserialize(&mut woopool_account_from.data.as_ref())?;        
        let woopool_account_to = client.get_account(&token_to_params.woopool)?;
        let woopool_to = WooPool::try_deserialize(&mut woopool_account_to.data.as_ref())?;
        let woopool_account_quote = client.get_account(&token_quote_params.woopool)?;
        let woopool_quote = WooPool::try_deserialize(&mut woopool_account_quote.data.as_ref())?;
        
        query_instruction(
            token_from_params.wooconfig,
            token_from_params.wooracle,
            token_from_params.woopool,
            token_from_accounts.price_update,
            token_to_params.wooracle,
            token_to_params.woopool,
            token_to_accounts.price_update,
            token_quote_accounts.price_update,
            woopool_from.token_vault,
            woopool_to.token_vault,
            token_quote_params.woopool,
            woopool_quote.token_vault,
            from_amount,
            min_to_amount
        )
    }

    pub fn swap(
        client: &RpcClient,
        payer: Pubkey,
        token_from: SupportedTokens,
        token_to: SupportedTokens,
        token_owner_account_from: Pubkey,
        token_owner_account_to: Pubkey,
        rebate_to: Pubkey,
        from_amount: u128,
        min_to_amount: u128
    ) -> Result<Instruction> {
        if token_from == token_to {
            return Err(anyhow!("From token cannot be same with To token"));
        }

        let token_from_accounts = get_woofi_accounts(&token_from);
        let token_to_accounts = get_woofi_accounts(&token_to);
        let token_from_params = get_woofi_pool_params(&token_from_accounts);
        let token_to_params = get_woofi_pool_params(&token_to_accounts);
        let token_quote_accounts = get_woofi_accounts(&QUOTE_TOKEN);
        let token_quote_params = get_woofi_pool_params(&token_quote_accounts);

        let woopool_account_from = client.get_account(&token_from_params.woopool)?;
        let woopool_from = WooPool::try_deserialize(&mut woopool_account_from.data.as_ref())?;        
        let woopool_account_to = client.get_account(&token_to_params.woopool)?;
        let woopool_to = WooPool::try_deserialize(&mut woopool_account_to.data.as_ref())?;
        let woopool_account_quote = client.get_account(&token_quote_params.woopool)?;
        let woopool_quote = WooPool::try_deserialize(&mut woopool_account_quote.data.as_ref())?;

        swap_instruction(
            payer,
            token_from_params.wooconfig,
            token_from_params.wooracle,
            token_from_params.woopool,
            token_from_accounts.price_update,
            token_to_params.wooracle,
            token_to_params.woopool,
            token_to_accounts.price_update,
            token_quote_accounts.price_update,
            woopool_from.token_vault,
            woopool_to.token_vault,
            token_quote_params.woopool,
            woopool_quote.token_vault,
            token_owner_account_from,
            token_owner_account_to,
            rebate_to,
            from_amount,
            min_to_amount
        )

    }


}
