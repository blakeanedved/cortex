mod types;
mod lang;

//use peg::Parse;
use crate::lang::grammar;

fn main() -> anyhow::Result<()> {
    let gamer = "{x::[1,2,3] | x == 3, x < 3}";
    let pos = dbg!(grammar::list_generator(gamer))?;
    //dbg!(gamer.position_repr(pos.0));

    Ok(())
}
