use anyhow::format_err;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

pub mod instructions;

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    pub http_url: String,
    pub ws_url: String,
    pub payer_path: String,
    pub admin_path: String,
    pub raydium_v3_program: Pubkey,
    pub slippage: f64,
    pub amm_config_key: Pubkey,

    pub mint0: Option<Pubkey>,
    pub mint1: Option<Pubkey>,
    pub pool_id_account: Option<Pubkey>,
    pub tickarray_bitmap_extension: Option<Pubkey>,
    pub amm_config_index: u16,
}

pub fn read_keypair_file(s: &str) -> anyhow::Result<Keypair> {
    solana_sdk::signature::read_keypair_file(s)
        .map_err(|_| format_err!("failed to read keypair from {}", s))
}
