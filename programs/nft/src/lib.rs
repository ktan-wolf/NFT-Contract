use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod events;
pub use events::*; 

declare_id!("C1sPwvGxxsM1vGjwVdJXYrLmVvRGyvWAoLFBZejiNEum");

#[program]
pub mod nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize> , name : String , fee : u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;

        emit!(InitializeEvent{
            admin: ctx.accounts.admin.key(),
            fee: ctx.accounts.marketplace.fee,
            marketplace: ctx.accounts.marketplace.key(),
            treasury: ctx.accounts.treasury.key(),
            reward_mint: ctx.accounts.reward_mint.key(),
            name: ctx.accounts.marketplace.name.clone(),
        });

        Ok(())
    }
}

