use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow_mod {
    use super::*;

    pub fn lock_sol(ctx: Context<LockSOL>, amount: u64) -> Result<()> {
        let sender = &ctx.accounts.bounty_account.clone();
        let bounty_account = &mut ctx.accounts.bounty_account;
        bounty_account.is_claimed = false;
        bounty_account.bump = *ctx.bumps.get("bounty_account").unwrap();
        bounty_account.amount = amount;
        bounty_account.is_active = true;
        let ix = system_instruction::transfer(
            ctx.accounts.authority.key,
            &sender.to_account_info().key(),
            amount,
        );

        invoke_signed(
            &ix,
            &[
                ctx.accounts.authority.to_account_info(),
                sender.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[b"bounty", &[bounty_account.bump]]],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockSOL<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init,payer=authority,seeds=[b"bounty",authority.key().as_ref()],bump)]
    pub bounty_account: Account<'info, BountyAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct BountyAccount {
    pub authority: Pubkey,
    pub amount: u64,
    pub is_claimed: bool,
    pub is_active: bool,
    pub bump: u8,
}
