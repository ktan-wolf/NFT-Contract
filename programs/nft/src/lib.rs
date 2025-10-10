use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod events;
pub use events::*; 

pub mod constants;
pub use constants::*;

pub mod error;

declare_id!("3AtZnEXfmtdb5WSyaNF8XJsDxWju4YDAQJiDycm7UqEN");

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

    pub fn list(ctx: Context<List> , price : u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;

        emit!(ListEvent {
            maker: ctx.accounts.maker.key(),
            maker_ata: ctx.accounts.maker_ata.key(),
            marketplace: ctx.accounts.marketplace.key(),
            maker_mint: ctx.accounts.maker_mint.key(),
            vault: ctx.accounts.vault.key(),
            collection_mint: ctx.accounts.collection_mint.key(),
            listing: ctx.accounts.listing.key(),
            name: ctx.accounts.marketplace.name.clone(),
            price: ctx.accounts.listing.price,
            fee: ctx.accounts.marketplace.fee,
        });

        Ok(())
    }

    pub fn purchase(ctx : Context<Purchase>) -> Result<()> {
        ctx.accounts.purchase()?;

        emit!(PurchaseEvent {
            maker: ctx.accounts.maker.key(),
            taker: ctx.accounts.taker.key(),
            taker_ata: ctx.accounts.taker_ata.key(),
            marketplace: ctx.accounts.marketplace.key(),
            treasury: ctx.accounts.treasury.key(),
            reward_mint: ctx.accounts.reward_mint.key(),
            maker_rewards_ata: ctx.accounts.maker_rewards_ata.key(),
            taker_rewards_ata: ctx.accounts.taker_rewards_ata.key(),
            maker_mint: ctx.accounts.maker_mint.key(),
            vault: ctx.accounts.vault.key(),
            listing: ctx.accounts.listing.key(),
            name: ctx.accounts.marketplace.name.clone(),
            price: ctx.accounts.listing.price,
            fee: ctx.accounts.marketplace.fee,
        });

        Ok(())
    }

    pub fn delist(ctx:Context<Delist>) -> Result<()>{
        ctx.accounts.refund_nft()?;
        ctx.accounts.close_mint_vault()?;
   
        emit!(DelistEvent {
            maker: ctx.accounts.maker.key(),
            maker_ata: ctx.accounts.maker_ata.key(),
            marketplace: ctx.accounts.marketplace.key(),
            maker_mint: ctx.accounts.maker_mint.key(),
            vault: ctx.accounts.vault.key(),
            collection_mint: ctx.accounts.collection_mint.key(),
            listing: ctx.accounts.listing.key(),
            name: ctx.accounts.marketplace.name.clone(),
            price: ctx.accounts.listing.price,
            fee: ctx.accounts.marketplace.fee,
        });

        Ok(())
    }
}

