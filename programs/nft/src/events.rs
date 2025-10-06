use anchor_lang::prelude::*;

#[event]
pub struct InitializeEvent{
    pub admin : Pubkey,
    pub marketplace : Pubkey,
    pub treasury : Pubkey,
    pub reward_mint : Pubkey,
    pub name : String,
    pub fee : u16,
}