use anchor_lang::prelude::*;

declare_id!("C2jWb5eavSTTzLrA9tBs1tzrkkKECcMJostkFeCZkX8f");

#[program]
pub mod light_switch {
    use anchor_lang::accounts;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let light_switch = &mut ctx.accounts.light_switch;
        light_switch.is_on = false;
        msg!("Light switch initialized. State: OFF");
        Ok(())
    }

    pub fn turn_on(ctx: Context<Toggle>) -> Result<()> {
        let light_switch = &mut ctx.accounts.light_switch;
        light_switch.is_on = true;
        msg!("Light turned ON!");
        Ok(())
    }

    pub fn turn_off(ctx: Context<Toggle>) -> Result<()> {
        let light_switch = &mut ctx.accounts.light_switch;
        light_switch.is_on = false;
        msg!("Light turned OFF!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1)]
    pub light_switch: Account<'info, LightSwitch>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Toggle<'info> {
    #[account(mut)]
    light_switch: Account<'info, LightSwitch>,
}

#[account]
pub struct LightSwitch {
    pub is_on: bool,
}
