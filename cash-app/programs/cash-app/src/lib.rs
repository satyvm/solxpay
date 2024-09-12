use anchor_lang::prelude::*;

declare_id!("9eWytELewtgDVotFj1tA3HxgKHgMN18oYRpg8R5MkJrs");

#[program]
pub mod cash_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
