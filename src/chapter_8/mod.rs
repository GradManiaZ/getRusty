
//use std::collections::;


pub fn this_is_chap_8()
{
  let mut my_vec: Vec<i32> = vec![1,2,3,4];
  my_vec.pop();
  my_vec.pop();
  my_vec.pop();
  dbg!(&my_vec);
  my_vec.push(2);
  my_vec.push(3);
  my_vec.push(4);
  dbg!(&my_vec);
  let third = &my_vec[2]; // danger zone

  let third: Option<&i32> = my_vec.get(2);

  match third {
    None => {
      println!("Out of bounds, third index empty");
    },
    Some (third) => 
    {
      println!("Third index value: {}", third); //i32
    }
  }

  
  if let Some(third) = my_vec.get(2)
  {
    let command = "if let Some(third) = my_vec.get(2)";
    
    println!("The condition {:?} is only true if .get() returns a valid i32", command)

  }
  else {
    let command: &str = "if let Some(third) = my_vec.get(2)";
    println!("The condition {:?} is only true if .get() returns None", command)
  }

  // better way of writing this
}