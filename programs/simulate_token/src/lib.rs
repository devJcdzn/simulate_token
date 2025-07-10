use anchor_lang::prelude::*;

declare_id!("6PaDhwPoVk66ZMbund3sp5q9UU8SP28upvpKHEZgrZLB");

#[program]
pub mod simulate_token {
    use super::*;

    pub fn resgister_token(
        ctx: Context<RegisterToken>,
        name: String,
        symbol: String,
        supply: u64,
    ) -> Result<()> {
        require!(name.len() <= 32, CustomError::NameTooLong);
        require!(symbol.len() <= 5, CustomError::SymbolTooLong);
        let metadata = &mut ctx.accounts.token_metadata;
        metadata.name = name;
        metadata.symbol = symbol;
        metadata.total_supply = supply;
        metadata.creator = ctx.accounts.creator.key();

        let user_token = &mut ctx.accounts.user_token_account;
        user_token.owner = ctx.accounts.creator.key();
        user_token.token = ctx.accounts.token_metadata.key();
        user_token.balance = supply;

        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let sender = &mut ctx.accounts.sender_account;
        let receiver = &mut ctx.accounts.receiver_account;

        require!(sender.balance >= amount, CustomError::InsufficientFunds);
        require!(sender.token == receiver.token, CustomError::TokenMismatch);

        sender.balance -= amount;
        receiver.balance += amount;

        Ok(())
    }

    pub fn create_user_token_account(ctx: Context<CreateUserTokenAccount>) -> Result<()> {
        let account = &mut ctx.accounts.user_token_account;
        account.owner = ctx.accounts.user.key();
        account.token = ctx.accounts.token.key();
        account.balance = 0;

        Ok(())
    }
}

#[account]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub creator: Pubkey,
}

#[account]
pub struct UserTokenAccount {
    pub owner: Pubkey,
    pub token: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct RegisterToken<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 10 + 8 + 32)]
    pub token_metadata: Account<'info, TokenMetadata>,

    #[account(init, payer = creator, space = 8 + 32 + 32 + 8)]
    pub user_token_account: Account<'info, UserTokenAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one = owner)]
    pub sender_account: Account<'info, UserTokenAccount>,

    #[account(mut)]
    pub receiver_account: Account<'info, UserTokenAccount>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateUserTokenAccount<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub user_token_account: Account<'info, UserTokenAccount>,

    pub token: Account<'info, TokenMetadata>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("Nome muito longo (máximo: 32 caracteres).")]
    NameTooLong,
    #[msg("Símbolo muito longo (máximo: 10 caracteres).")]
    SymbolTooLong,
    #[msg("Saldo insuficiente para transferência.")]
    InsufficientFunds,
    #[msg("Contas de token incompatíveis.")]
    TokenMismatch,
}

pub struct Initialize {}
