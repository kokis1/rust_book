//! # This is a simple (example) Art Library
//! here lays out the internal structure of the crate, which will then be altered
//! for the public use with ```use``` statements

// bringing these into the base scope with a pub use statement
pub use kinds::PrimaryColour;
pub use kinds::SecondaryColour;
pub use utils::mix;


pub mod kinds {

   /// The primary colours according to the RYB model
   pub enum PrimaryColour {
      Red,
      Blue,
      Yellow,
   }

   /// The secondary colours according to the RYB model
   pub enum SecondaryColour {
      Orange,
      Green,
      Purple,
   }

}

pub mod utils {
   use crate::kinds::*;

   /// Combines two primary colours in equal amounts to
   /// create a secondary colour.
   pub fn mix(c1: PrimaryColour, c2: SecondaryColour) {
      todo!();
   }
}