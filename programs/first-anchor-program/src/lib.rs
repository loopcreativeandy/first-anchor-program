use anchor_lang::prelude::*;

declare_id!("AProvwUdgUYhvLJjejBH5Ba7LfS8wKAbj5KmFNmf2FNt");

#[program]
pub mod first_anchor_program {
    use super::*;

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
    
    pub fn my_close(ctx: Context<CloseAccounts>, _pda_nr: u32) -> Result<()> {
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
