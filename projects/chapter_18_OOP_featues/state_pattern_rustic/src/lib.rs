pub mod blog {

   // this version makes use of the type system to simplify the state pattern
   // however, this isn't a pure OO implementation, nor is it completely the state pattern
   // because each state is it's own type, not an attribute of a parent type
   pub struct Post {
      content: String,
   }
   pub struct DraftPost {
      content: String,
   }
   pub struct PendingReviewPost {
      content: String,
   }
   impl Post {
      pub fn new() -> DraftPost {
         DraftPost {
            content: String::new(),
         }
      }
      pub fn content(&self) -> &str {
         &self.content
      }
   }
   impl DraftPost {
      pub fn add_text(&mut self, text: &str) {
         self.content.push_str(text);
      }
      // self becomes a different type, so no reference needed
      //                      ↓
      pub fn request_review(self) -> PendingReviewPost {
         PendingReviewPost {
            content: self.content,
         }
      }
   }
   impl PendingReviewPost {
      pub fn approve(self) -> Post {
         Post {
            content: self.content
         }
      }
   }
}