//! # art
//!
//! A library for modeling artistic concepts.
//! 

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds{
    /// The primary colors according to the RYB color modal.
    pub enum PrimaryColor{
        Red,
        Yellow,
        Blue
    }
    /// The secondary colors according to the RYB color modal.
    pub enum SecondaryColor{
        Orange,
        Green,
        Purple
    }
}

pub mod utils{
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor{
        //--snip--
        //ANCHOR_END: here
        SecondaryColor::Orange
        //Anchor: here
    }
}