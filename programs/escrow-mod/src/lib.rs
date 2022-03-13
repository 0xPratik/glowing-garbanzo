use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
// use anchor_spl::token::{self, SetAuthority, Token, TokenAccount, Transfer};

declare_id!("EtxmAK5Hio4p1NQEwrRJu6UJpheKpBf1cfg9U7x3gGEX");

#[program]
pub mod escrow_mod {
    use super::*;

    pub fn lock_sol(ctx: Context<LockSOL>, amount: u64) -> Result<()> {
        let sender = &ctx.accounts.bounty_account.clone();
        let bounty_account = &mut ctx.accounts.bounty_account;
        bounty_account.authority = ctx.accounts.authority.to_account_info().key();
        bounty_account.bump = *ctx.bumps.get("bounty_account").unwrap();
        bounty_account.amount = amount;
        let ix = system_instruction::transfer(
            ctx.accounts.authority.key,
            &sender.to_account_info().key(),
            amount,
        );

        invoke(
            &ix,
            &[
                ctx.accounts.authority.to_account_info(),
                sender.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }

    // pub fn Lock_spl(ctx: Context<LockSPL>) -> Result<()> {
    //     Ok(())
    // }

    pub fn claim_bounty(ctx: Context<ClaimBounty>) -> Result<()> {
        msg!("SHIT ITS WORKING");
        **ctx
            .accounts
            .bounty_account
            .to_account_info()
            .try_borrow_mut_lamports()? = ctx
            .accounts
            .bounty_account
            .to_account_info()
            .lamports()
            .checked_sub(ctx.accounts.bounty_account.amount)
            .ok_or(ProgramError::InvalidArgument)?;
        **ctx
            .accounts
            .reciever_account
            .to_account_info()
            .try_borrow_mut_lamports()? = ctx
            .accounts
            .reciever_account
            .to_account_info()
            .lamports()
            .checked_add(ctx.accounts.bounty_account.amount)
            .ok_or(ProgramError::InvalidArgument)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockSOL<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init,payer=authority,seeds=[b"bounty".as_ref(),authority.key().as_ref()],bump)]
    pub bounty_account: Account<'info, BountyAccount>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct LockSPL<'info> {
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
// }

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,seeds=[b"bounty",authority.key().as_ref()],bump=bounty_account.bump,has_one=authority)]
    pub bounty_account: Account<'info, BountyAccount>,
    #[account(mut)]
    pub reciever_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct BountyAccount {
    pub authority: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
