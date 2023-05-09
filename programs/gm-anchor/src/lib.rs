use anchor_lang::prelude::*;

declare_id!("GMoMVeaVmD8H2JqgbSaSBuELofJus3z5Lb97AfKFrw3k");

#[program]
pub mod gm_anchor {
    use super::*;

    pub fn gm_instruction(ctx: Context<GmAccounts>, gms: u8) -> Result<()> {

        msg!("signer signed is {}", ctx.accounts.signer.is_signer);

        require_gte!(10, gms, GmErrors::TooManyGMs);
        require!(gms <= 10, GmErrors::TooManyGMs);
        if gms > 10 {
            // return Err(error!(GmErrors::TooManyGMs));
            return err!(GmErrors::TooManyGMs);
        }
        msg!("GM!");
        if gms>1 {
            gm_instruction(ctx, gms-1)
        } else {
            Ok(())
        }
    }
}

#[error_code]
pub enum GmErrors{
    #[msg("Too many GMs requested! Maximum is 10 GMs")]
    TooManyGMs

}

#[derive(Clone)]
pub struct GmProgram;
impl anchor_lang::Id for GmProgram {
    fn id() -> Pubkey {
        id()
    }
}

#[derive(Accounts)]
pub struct GmAccounts<'info> {
    /// CHECK: this is save, trust me, I'm a dev!
    pub signer: UncheckedAccount<'info>,
    //pub gm_program: Program<'info, GmProgram>
}
