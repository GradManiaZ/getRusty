/*
 *  Chapter 5
 */

 //Definition and Instantiation

use std::f32::consts::TAU;

 #[allow(dead_code)]

 #[derive(Debug)]
pub struct ExampleUser{
  active:bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

/* Impl provides a section for struct method syntax 
 * Impliminations are also used in enums and traits, to be covered later.
*/
impl ExampleUser
{
  
  pub fn build_user (email: String, username: String) -> ExampleUser
  {
    ExampleUser {
      active: true,
      username: username,
      email: email,
      sign_in_count: 0 
    }
  }
}

// example two

#[derive(Debug)]
pub struct Rectangle {
  width: u32, // negatuves dont make sense 
  height: u32,
}
impl Rectangle {
    pub fn new(width: u32, height:u32) -> Self //I'm still not sure why this is capitalised
    {
      Rectangle{width:width,height:height}
    }
    pub fn get_area(&self) -> u32
    {
      self.width * self.height
    }
    pub fn can_hold(&self, other_Rec:&Rectangle) -> bool
    {
      self.get_area() >= other_Rec.get_area()
    }
}

// ###############    ENUMS   ###############

pub enum IdAddrKind
{
  V4(u8,u8,u8,u8),
  V6(String)
}
pub struct IpAddr
{
  ip:String,
  kind: IdAddrKind,
}
impl IpAddr
{
  pub fn route()
  {
    let home_ip: IdAddrKind = IdAddrKind::V4(127, 0, 0, 1);
    let loopback:IdAddrKind = IdAddrKind::V6(String::from("::1"));
  }
}

// ###############    Lets Expand   ###############

pub enum Message
{
  Quit,
  Move{x:i32, y:i32},
  Write(String),
  ChangeColour(i32,i32,i32,),
}

impl Message {
    pub fn call(self)
    {
      match self
      {
        Message::Quit=> {
          
        },
        Message::Move{x,y}=> {
          println!("New X:{} New Y: {}", x,y)
        },
        Message::Write(msg)=> {
          println!("{}",msg)
        },
        Message::ChangeColour(r_Val ,b_Val, g_Val)=> {
          println!("R:{} G:{} B:{}",r_Val,b_Val,g_Val)
        },
      }
    }
}

pub fn random_foo()
{
  let x: u8 = 3u8;
}

// Handling null reference using the option type

enum BeginTransfrom<PositionAugment> // this enum is already defined in the standard library
{
    None,
    Some(PositionAugment)
}
impl BeginTransfrom<(i32,i32,i32)>
{
  pub fn show(self)
  {
    match self
    {
      BeginTransfrom::None => {},
      BeginTransfrom::Some(absoulte_adjustment) =>
      {
        let (x,y,z) = absoulte_adjustment;
        println!("x:{},y:{},z:{}",x,y,z);
      }
    }
  }
}


impl BeginTransfrom<i32>
{

}
