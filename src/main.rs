mod types;
mod lang;

//use peg::Parse;
use crate::lang::grammar;

fn main() -> anyhow::Result<()> {
    let gamer = "{(x,y)::[(1,2),(2,3),(3,4)] | x == 3}";
    let _pos = dbg!(grammar::list_generator(gamer))?;
    //dbg!(gamer.position_repr(pos.0));

    Ok(())
}
