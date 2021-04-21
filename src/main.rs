mod types;
mod lang;

use crate::lang::grammar;

fn main() -> anyhow::Result<()> {
    dbg!(grammar::expression("1+2*3-4/5"))?;

    Ok(())
}
