use anchor_lang::prelude::*;

declare_id!("C2jWb5eavSTTzLrA9tBs1tzrkkKECcMJostkFeCZkX8f");

#[program]
pub mod light_switch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
