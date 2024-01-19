mod combination;

mod loadcase;
mod material;
mod member;
mod member_result;
mod node;
mod node_reaction;
mod section;
mod settings;

pub use combination::*;
pub use loadcase::*;
pub use material::*;
pub use member::*;
pub use member_result::*;
pub use node::*;
pub use node_reaction::*;
pub use section::*;
pub use settings::*;

#[derive(Debug, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Direction {
    None = 0,
    X,
    Y,
    Z,
}
