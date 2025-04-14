use std::{error::Error, fmt, str::FromStr};
use anyhow::anyhow;


use solana_program::{
    pubkey,
    pubkey::Pubkey,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SupportedTokens {
    SOL,
    USDC
}

impl fmt::Display for SupportedTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedTokens::SOL => {
                write!(f, "SOL")
            },
            SupportedTokens::USDC => {
                write!(f, "USDC")
            },
        }
    }
}

impl Error for SupportedTokens {}

impl FromStr for SupportedTokens {

    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<SupportedTokens, Self::Err> {
        match input {
            "SOL"  => Ok(SupportedTokens::SOL),
            "sol"  => Ok(SupportedTokens::SOL),
            "USDC"  => Ok(SupportedTokens::USDC),
            "usdc"  => Ok(SupportedTokens::USDC),
            _      => Err(anyhow!("Not supported token")),
        }
    }
}

pub struct TokenAccounts {
    pub token_mint: Pubkey,
    pub feed_account: Pubkey,
    pub price_update: Pubkey,
    pub quote_token_mint: Pubkey
}

pub struct PoolParams {
    pub wooconfig: Pubkey,
    pub wooracle: Pubkey,
    pub woopool: Pubkey
    // token_vault need client.get_account(woopool) to get the real account
    // pub token_vault: Pubkey
}

pub const QUOTE_TOKEN: SupportedTokens = SupportedTokens::USDC;

// PROD
pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

// DEV
// pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
// pub const USDC: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

pub const QUOTE_MINT: Pubkey = USDC;

pub const SOL_FEED_ACCOUNT: Pubkey = pubkey!("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG");
pub const USDC_FEED_ACCOUNT: Pubkey = pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");

pub const SOL_PRICE_UPDATE: Pubkey = pubkey!("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");
pub const USDC_PRICE_UPDATE: Pubkey = pubkey!("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX");

pub fn get_wooconfig_address(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooconfig"], program_id)
}

pub fn get_wooracle_address(wooconfig: &Pubkey, token_mint: &Pubkey, feed_account: &Pubkey, price_update: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooracle", wooconfig.as_ref(), token_mint.as_ref(), feed_account.as_ref(), price_update.as_ref()], program_id)
}

pub fn get_woopool_address(wooconfig: &Pubkey, token_mint: &Pubkey, quote_token_mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"woopool", wooconfig.as_ref(), token_mint.as_ref(), quote_token_mint.as_ref()], program_id)
}

pub fn get_wooammpool_address(wooconfig: &Pubkey, token_mint_a: &Pubkey, token_mint_b: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"wooammpool", wooconfig.as_ref(), token_mint_a.as_ref(), token_mint_b.as_ref()], program_id)
}

pub fn get_woofi_accounts(token: &SupportedTokens) -> TokenAccounts {
    match token {
        SupportedTokens::SOL => TokenAccounts {
            token_mint: SOL,
            feed_account: SOL_FEED_ACCOUNT,
            price_update: SOL_PRICE_UPDATE,
            quote_token_mint: USDC
        },
        SupportedTokens::USDC => TokenAccounts {
            token_mint: USDC,
            feed_account: USDC_FEED_ACCOUNT,
            price_update: USDC_PRICE_UPDATE,
            quote_token_mint: USDC
        },
    }
}

pub fn get_woofi_pool_params(token_accounts: &TokenAccounts) -> PoolParams {
    let program_id = &woofi::id();
    let wooconfig = get_wooconfig_address(program_id).0;
    let wooracle = get_wooracle_address(&wooconfig, &token_accounts.token_mint, &token_accounts.feed_account, &token_accounts.price_update, program_id).0;
    let woopool = get_woopool_address(&wooconfig, &token_accounts.token_mint, &token_accounts.quote_token_mint, program_id).0;

    return PoolParams {
        wooconfig,
        wooracle,
        woopool
    };
}
