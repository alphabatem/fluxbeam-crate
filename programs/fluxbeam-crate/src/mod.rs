use solana_program::pubkey::Pubkey;

pub mod create_pool;
pub mod add_liquidity;
pub mod remove_liquidity;
pub mod add_liquidity_single;
pub mod remove_liquidity_single;
pub mod swap;
pub mod interfaces;

#[derive(Clone)]
pub struct FluxBeam;

impl anchor_lang::Id for FluxBeam {
    fn id() -> Pubkey {
        Pubkey::try_from("FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X").unwrap()
    }
}