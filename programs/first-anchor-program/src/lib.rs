use anchor_lang::prelude::*;

declare_id!("AProvwUdgUYhvLJjejBH5Ba7LfS8wKAbj5KmFNmf2FNt");

#[program]
pub mod first_anchor_program {
    use super::*;

    pub fn my_instruction(ctx: Context<InstructionAccounts>, input_number: u64) -> Result<()> {
        ctx.accounts.data_account.number1 = input_number;
        ctx.accounts.data_account.number2 = 2;
        msg!("Data account created!");
        Ok(())
    }
    
    pub fn my_sum(ctx: Context<SumAccounts>) -> Result<()> {
        let sum = ctx.accounts.data_account_1.number1 + ctx.accounts.data_account_2.number1;
        msg!("The sum is {}", sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(init, payer = user, space = 8 + 8 + 2)]
    pub data_account: Account<'info, DifferentAccountStruct>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SumAccounts<'info> {
    pub data_account_1: Account<'info, DifferentAccountStruct>,
    pub data_account_2: Account<'info, DifferentAccountStruct>,
}

#[account]
pub struct AccountStruct {
    number1: u64,
    number2: u16
}
#[account]
pub struct DifferentAccountStruct {
    number1: u64,
    number2: u16
}
