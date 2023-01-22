use crate::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
const HYDRA_ID: Pubkey = Pubkey::new_from_array([
    10, 126, 203,  31, 202, 195,  95,  27,
   175, 237,  98,  15, 124,  12, 118, 169,
    56, 198, 194, 208,   7,  76, 111,  47,
   128,  57,  55, 192,  81, 109,  36, 203
 ]);
#[derive(Accounts)]
#[instruction(params: HouseInitParams)] // rpc parameters hint
pub struct HouseInit<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<HouseState>(),
        payer = payer,
        seeds = [HOUSE_SEED, authority.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub house: AccountLoader<'info, HouseState>,
    /// CHECK:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,

    pub switchboard_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = 
            switchboard_queue.load()?.unpermissioned_vrf_enabled == true @ VrfFlipError::OracleQueueRequiresPermissions
    )]
    pub switchboard_queue: AccountLoader<'info, OracleQueueAccountData>,

    #[account(
        /*
        init_if_needed,
        payer = payer,
        mint::decimals = 2,
        mint::authority = house,
        mint::freeze_authority = house, */
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = house,
    )]
    pub house_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK:
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
    /// CHECK:
    #[account(owner = HYDRA_ID)]
    pub hydra: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct HouseInitParams {}

impl HouseInit<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &HouseInitParams,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, _params: &HouseInitParams) -> anchor_lang::Result<()> {
        msg!("house_init");

        let house_bump = ctx.bumps.get("house").unwrap().clone();
        
        let house = &mut ctx.accounts.house.load_init()?;
        house.bump = house_bump;
        house.authority = ctx.accounts.authority.key().clone();
        house.switchboard_mint = ctx.accounts.switchboard_mint.key().clone();
        house.mint = ctx.accounts.mint.key().clone();
        house.switchboard_queue = ctx.accounts.switchboard_queue.key().clone();
        house.house_vault = ctx.accounts.house_vault.key().clone();
        house.hydra = ctx.accounts.hydra.key().clone();
        drop(house);

        Ok(())
    }
}
