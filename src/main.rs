use chapter_5_6::Message;

use crate::chapter_5_6::{ExampleUser, Rectangle};


mod chapter_5_6;
fn main() {
    // println!("Welcome to the rust source directory");
    // let user = ExampleUser::build_user("example_email".to_owned(), "exmaple_username".to_owned());

    // println!("New user created: {:?}", user );

    // let rectangle = Rectangle::new(50, 60);
    // let rectangle_2 = Rectangle::new(21, 32);

    // println!("The area of the rectangle is {}", &rectangle.get_area());
    // println!("New Rec Created {:?}", rectangle_2);
    // let rec_one_two_holder_state = if rectangle.can_hold(&rectangle_2) {"can"} else {"can't"};
    // println!("Rec One, {} hold Rec Two", rec_one_two_holder_state);

    let message = Message::Write(String::from("value"));
    message.call();
}
