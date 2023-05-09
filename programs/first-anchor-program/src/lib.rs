use anchor_lang::prelude::*;
use gm_anchor;
// use solana_program;

declare_id!("AProvwUdgUYhvLJjejBH5Ba7LfS8wKAbj5KmFNmf2FNt");

// #[derive(Clone)]
// pub struct GmProgram;
// impl anchor_lang::Id for GmProgram {
//     fn id() -> Pubkey {
//         gm_anchor::id()
//     }
// }

#[program]
pub mod first_anchor_program {
    use super::*;

    pub fn my_gm_instruction(ctx: Context<MyGmAccounts>) -> Result<()>{
        let gms = 2u8;
        let seeds: &[&[&[u8]]] = &[&[b"authority", &[*ctx.bumps.get("pda").unwrap()]]];
        // let accounts = PseudoGmAccounts{
        //     signer: ctx.accounts.pda
        // };
        let mut signer = ctx.accounts.pda.to_account_info();
        signer.is_signer = true;
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.gm_program.to_account_info(), 
            gm_anchor::cpi::accounts::GmAccounts{
                signer: signer
            },
            seeds
        );
        msg!(" is signer {}", cpi_context.accounts.signer.is_signer);
        gm_anchor::cpi::gm_instruction(
            cpi_context,
            gms
        )?;
        msg!("cpi successful");
        Ok(())

        solana_program::program::invoke_signed(
            &solana_program::instruction::Instruction{
                program_id: ctx.accounts.gm_program.key(),
                accounts: vec![solana_program::instruction::AccountMeta::new_readonly
                    (ctx.accounts.pda.key(), true)],
                    // sha256(global:gm_instruction) -> 515D7D4C030E63C0
                data: vec![81, 93, 125, 76, 3, 14, 99, 192, gms]
            }, 
            &[
                ctx.accounts.pda.to_account_info()
            ],
            seeds
        ).map_err(Into::into)

    }
    
    pub fn my_instruction(ctx: Context<InstructionAccounts>, input_number: u64, _pda_nr: u32) -> Result<()> {
        ctx.accounts.data_account.number1 = input_number;
        ctx.accounts.data_account.number2 = 2;
        // ctx.accounts.data_account.bump = *ctx.bumps.get("data_account").unwrap();
        msg!("Data account created!");
        Ok(())
    }
    
    pub fn my_sum(ctx: Context<SumAccounts>, _pda1_nr: u32, _pda2_nr: u32) -> Result<()> {
        let sum = ctx.accounts.data_account_1.number1 + ctx.accounts.data_account_2.number1;
        msg!("The sum is {}", sum);
        Ok(())
    }
    
    pub fn my_close(_ctx: Context<CloseAccounts>, _pda_nr: u32) -> Result<()> {
        msg!("Data account closed!");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(input_number: u64, pda_nr: u32)]
pub struct InstructionAccounts<'info> {
    // #[account(seeds = [b"data", user.key().as_ref(), &pda_nr.to_le_bytes()], bump,
    //     mut, realloc = 8 + 8 + 2 + 1,
    //     realloc::payer = user,
    //     realloc::zero = false)]
    #[account(init, payer = user, space =  8 + 8 + 2 + 1)]
    pub data_account: Account<'info, DifferentAccountStruct>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(pda_nr: u32)]
pub struct CloseAccounts<'info> {
    #[account(mut, close = user)]//, seeds = [b"data", user.key().as_ref(), &pda_nr.to_le_bytes()], bump = data_account.bump)]
    pub data_account: Account<'info, DifferentAccountStruct>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct MyGmAccounts<'info> {
    /// CHECK: This account is never written to or read from (it doesn't even exist; it's just used for signing)
    #[account(seeds = [b"authority"], bump)]
    pub pda: UncheckedAccount<'info>,
    pub gm_program: Program<'info, gm_anchor::GmProgram>
}

#[derive(Accounts)]
#[instruction(pda1_nr: u32, pda2_nr: u32)]
pub struct SumAccounts<'info> {
    #[account(seeds = [b"data", user.key().as_ref(), &pda1_nr.to_le_bytes()], bump = data_account_1.bump)]
    pub data_account_1: Account<'info, DifferentAccountStruct>,
    #[account(seeds = [b"data", user.key().as_ref(), &pda2_nr.to_le_bytes()], bump = data_account_2.bump)]
    pub data_account_2: Account<'info, DifferentAccountStruct>,
    pub user: Signer<'info>,
}

#[account]
pub struct AccountStruct {
    number1: u64,
    number2: u16
}
#[account]
pub struct DifferentAccountStruct {
    number1: u64,
    number2: u16,
    bump: u8
}
