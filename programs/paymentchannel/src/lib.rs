use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id!("61XLPpfbHiT2gEQdWhBGGCfvdvSeRqT8MGwm4hjYNZG2");

//This smart contract assumes that multisig wallet exist and is being used to sign the transactions(with >1 signers) here.

#[program]
pub mod paymentchannel {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        user1_amount: u64,
        user2_amount: u64,
        exit_time: u64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp as u64;
        require!(now > exit_time, ChannelError::InvalidTime);
        let channel = &mut ctx.accounts.channel;
        require!(!channel.initialized, ChannelError::InvalidChannel);
        channel.user1 = *ctx.accounts.user1.key;
        channel.user2 = *ctx.accounts.user2.key;
        solana_program::system_instruction::transfer(&channel.user1, &channel.key(), user1_amount);
        solana_program::system_instruction::transfer(&channel.user2, &channel.key(), user2_amount);
        channel.user1_balance = user1_amount;
        channel.user2_balance = user2_amount;
        channel.exit_time = exit_time;
        channel.initialized = true;
        channel.exited = false;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, user1_amount: u64, user2_amount: u64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp as u64;
        let channel = &mut ctx.accounts.channel;
        require!(!channel.exited, ChannelError::ChannelExited);
        require!(now < channel.exit_time, ChannelError::InvalidTime);
        require!(
            (channel.user1 == *ctx.accounts.user1.key)
                && (channel.user2 == *ctx.accounts.user2.key),
            ChannelError::InvalidSigner
        );
        require!(
            channel.user1_balance + channel.user2_balance == user1_amount + user2_amount,
            ChannelError::InvalidUpdatedBalances
        );
        channel.user1_balance = user1_amount;
        channel.user2_balance = user2_amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let channel = &mut ctx.accounts.channel;
        require!(!channel.exited, ChannelError::ChannelExited);
        solana_program::system_instruction::transfer(&channel.key(), &channel.user1, channel.user1_balance);
        solana_program::system_instruction::transfer(&channel.key(), &channel.user2, channel.user2_balance);
        channel.user1_balance=0;
        channel.user2_balance=0;
        channel.exited = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user1, space = 90)]
    pub channel: Account<'info, Channel>,
    #[account(mut)]
    pub user1: Signer<'info>,
    #[account(mut, constraint = user1.key != user2.key)]
    pub user2: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user1: Signer<'info>,
    #[account(mut, constraint = user1.key != user2.key)]
    pub user2: Signer<'info>,
    #[account(mut)]
    pub channel: Account<'info, Channel>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user1: Signer<'info>,
    #[account(mut, constraint = user1.key != user2.key)]
    pub user2: Signer<'info>,
    #[account(mut)]
    pub channel: Account<'info, Channel>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Channel {
    pub user1: Pubkey, //32
    pub user2: Pubkey,//32
    pub user1_balance: u64, //8
    pub user2_balance: u64, //8
    pub exit_time: u64, //8
    pub initialized: bool, //1
    pub exited: bool, //1
}

#[error_code]
pub enum ChannelError {
    #[msg("Invalid balance")]
    InvalidBalances,
    #[msg("Invalid Time")]
    InvalidTime,
    #[msg("Channel Expired, try calling withdraw instead")]
    ChannelExpired,
    #[msg("Channel Exited")]
    ChannelExited,
    #[msg("Already Initialized")]
    InvalidChannel,
    #[msg("Invalid Signer(s)")]
    InvalidSigner,
    #[msg("Invalid Updated Balances, do not match stored balance")]
    InvalidUpdatedBalances,
}
