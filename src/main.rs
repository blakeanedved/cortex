mod executor;
mod utils;
mod types;
mod lang;
pub mod math;

pub use crate::types::ASTNode;
pub use crate::lang::language;
pub use crate::utils::print_astnode;
pub use crate::executor::execute;
pub use crate::executor::insert_var;

fn main() {
    // insert_var(String::from("a"), ASTNode::new_number(42.0));
    print_astnode(&execute(&language::program("(0<>[1,2,3]<>[4,5,6])><[2,4,6]").unwrap()));
    // print_astnode(&language::program("2^2^3==2^6").unwrap());
}
