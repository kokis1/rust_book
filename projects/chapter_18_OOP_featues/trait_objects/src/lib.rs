// defining a trait object that must implement a draw() function
pub mod gui {
   pub trait Draw {
      fn draw(&self);
   }

   // defining a struct that contains components that implement Draw
   pub struct Screen {

      // the sizes of things that implement Draw are not known at compile time
      // therefore a collection of them must use pointers, in this case the Box (heap allocation)
      // dyn means any type that has Draw implemented
      pub components: Vec<Box<dyn Draw>>,
   }

   impl Screen {

      // calling draw on each component
      pub fn run(&self) {
         for component in self.components.iter() {
            component.draw();
         }
      }
   }

   // creating some structs that implement Draw
   pub struct Button {
      pub width: u32,
      pub height: u32,
      pub label: String,
   }

   impl Draw for Button {
      fn draw(&self) {
         println!("I'm a button, width: {}, height: {}, label: {}", self.width, self.width, self.label);
      }
   }

   pub struct SelectBox {
      pub width: u32,
      pub height: u32,
      pub options: Vec<String>,
   }

   impl Draw for SelectBox {
      fn draw(&self) {
         println!("I'm a selection box, width: {}, height: {}, options: {:?}", self.width, self.height, self.options);
      }
   }
}