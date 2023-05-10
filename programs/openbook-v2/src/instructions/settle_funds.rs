use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use fixed::types::I80F48;

use crate::accounts_ix::*;
use crate::state::*;

pub fn settle_funds<'info>(ctx: Context<'_, '_, '_, 'info, SettleFunds<'info>>) -> Result<()> {
    let mut open_orders_account = ctx.accounts.open_orders_account.load_full_mut()?;
    let mut position = &mut open_orders_account.fixed_mut().position;
    let mut market = ctx.accounts.market.load_mut()?;

    if ctx.remaining_accounts.is_empty() {
        market.quote_fees_accrued += position.referrer_rebates_accrued;
    }
    market.referrer_rebates_accrued -= position.referrer_rebates_accrued;
    market.base_deposit_total -= position.base_free_native.to_num::<u64>();
    market.quote_deposit_total -= position.quote_free_native.to_num::<u64>();

    let seeds = market_seeds!(market);
    let signer = &[&seeds[..]];

    drop(market);

    if !ctx.remaining_accounts.is_empty() && position.referrer_rebates_accrued > 0 {
        let referrer = ctx.remaining_accounts[0].to_account_info();
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.quote_vault.to_account_info(),
                to: referrer,
                authority: ctx.accounts.market.to_account_info(),
            },
        );
        token::transfer(
            cpi_context.with_signer(signer),
            position.referrer_rebates_accrued,
        )?;
    }

    if position.base_free_native > 0 {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.base_vault.to_account_info(),
                to: ctx.accounts.payer_base.to_account_info(),
                authority: ctx.accounts.market.to_account_info(),
            },
        );
        token::transfer(
            cpi_context.with_signer(signer),
            position.base_free_native.to_num(),
        )?;
    }

    if position.quote_free_native > 0 {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.quote_vault.to_account_info(),
                to: ctx.accounts.payer_quote.to_account_info(),
                authority: ctx.accounts.market.to_account_info(),
            },
        );
        token::transfer(
            cpi_context.with_signer(signer),
            position.quote_free_native.to_num(),
        )?;
    }

    // Set to 0 after transfer
    position.base_free_native = I80F48::ZERO;
    position.quote_free_native = I80F48::ZERO;
    position.referrer_rebates_accrued = 0;

    Ok(())
}
