use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::{self, Mint, SetAuthority, Token, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

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

    pub fn lock_spl(ctx: Context<LockSPL>, amount: u64) -> Result<()> {
        let bounty_account = &mut ctx.accounts.bounty_account;
        bounty_account.authority = ctx.accounts.authority.to_account_info().key();
        bounty_account.bump = *ctx.bumps.get("bounty_account").unwrap();
        bounty_account.vault_bump = *ctx.bumps.get("vault_account").unwrap();
        bounty_account.amount = amount;
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.authority_token_account.to_account_info(),
                    to: ctx.accounts.vault_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;

        // token::set_authority(
        //     CpiContext::new(
        //         ctx.accounts.token_program.to_account_info(),
        //         token::SetAuthority {
        //             current_authority: ctx.accounts.authority.to_account_info(),
        //             account_or_mint: ctx.accounts.authority_token_account.to_account_info(),
        //         },
        //     ),
        //     AuthorityType::AccountOwner,
        //     Some(vault_authority),
        // )?;
        Ok(())
    }

    pub fn unlock_spl(ctx: Context<UnLockSPL>) -> Result<()> {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_account.to_account_info(),
                    to: ctx.accounts.winner_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                &[&[
                    b"vault",
                    ctx.accounts.authority.key().as_ref(),
                    &[ctx.accounts.bounty_account.vault_bump],
                ]],
            ),
            ctx.accounts.bounty_account.amount,
        )?;
        Ok(())
    }

    pub fn claim_bounty(ctx: Context<UnLockSol>) -> Result<()> {
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

#[derive(Accounts)]
pub struct LockSPL<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init_if_needed, payer = authority,seeds=[b"vault",authority.key().as_ref()],bump,token::mint=mint,token::authority=authority)]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(init,payer=authority,seeds=[b"splbounty".as_ref(),authority.key().as_ref()],bump)]
    pub bounty_account: Account<'info, BountyAccount>,
    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UnLockSPL<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,seeds=[b"vault",authority.key().as_ref()],bump=bounty_account.vault_bump)]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(mut,seeds=[b"splbounty",authority.key().as_ref()],bump=bounty_account.bump,has_one=authority)]
    pub bounty_account: Account<'info, BountyAccount>,
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UnLockSol<'info> {
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
    pub vault_bump: u8,
}

#[account]
#[derive(Default)]
pub struct BountySPLAccount {
    pub authority: Pubkey,
    pub authority_token_account: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
