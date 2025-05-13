
use anchor_lang::prelude::*;

declare_id!("WEB11111111111111111111111111111111111111111");

#[program]
pub mod webstreet_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _bump: u8) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.staked_amount += amount;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let user = &mut ctx.accounts.user;
        require!(user.staked_amount >= amount, CustomError::InsufficientStake);
        user.staked_amount -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 8, seeds = [b"user", authority.key().as_ref()], bump)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
}

#[account]
pub struct User {
    pub staked_amount: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Insufficient staked balance")]
    InsufficientStake,
}
