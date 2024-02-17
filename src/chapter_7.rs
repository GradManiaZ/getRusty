
/**
 * Introduction to crate management
 * 
 *  Crate - "Smallest part of code that the rust compiler considers at a time"
 * 
 *  Binary Crate vs Library Crate
 * 
 *  Binary crates are programs that can compile into an executable such as command line program or a server, each must have a main
 * 
 *  Library crates dont have a main function and dont compile, instead they define functionality to be shared with multiple projects eg rand is a lib crate.
 * 
 *  Defining modules, scope contol and privacy
 */
mod foods;

pub use foods::vegetables::potato as MyPotato;

pub struct StandardFood
{
  pub quanitiy:i32,
  secret_sause: String
}

impl StandardFood
{
  pub fn new(quanitiy:i32, secret_str: String) -> Self{
    StandardFood{quanitiy:quanitiy, secret_sause:secret_str}
  }
  pub fn modify_sause(mut self, NewRecipe:String)
  {
    self.secret_sause = NewRecipe;
  }
  pub fn i_said_show_me_the_sause(self)
  {
    println!("{}",self.secret_sause);
  }
}
pub fn test_method(){
  foods::vegetables::potato::i_am_potato();
}