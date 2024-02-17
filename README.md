# getRusty
Documenting the learning journy through the rust book.

## Chapters

### Chapter 7

#### Defining Modules

public crates

``` Rust

  use rand;

```

Local crates

``` Rust

pub mod garden;
//or
use crate::garden::foods::vegetables::{Asparagus,Potato}; // direct path from root 

```


```Shell
backyard ------------------------------- "Root Folder"
├── Cargo.lock
├── Cargo.toml
└── src -------------------------------- "Source Folder"
     ├── garden ------------------------ "User Defined Module Folder"
     │      └── foods ------------------ "User Defined Sub-Module Folder"
     │      │     └── vegetables.rs ---- "User Defined Sub-Sub-Module"
     │      └── foods.rs --------------- "User Defined Sub-Module"
     ├── garden.rs --------------------- "User Defined Module"
     └── main.rs
```

#### Custom Scope

```shell
crate
└── pub garden
    └── pub foods
        ├── secret_food
        │   └── magic mushrooms
        └── pub vegetables
            ├── asparagus
            └── potato
```

Naming scopes takes many forms, in this case it referse to assigning a keyword to a module. Consider having to access potato through

```Rust
use foods::vegetables::potato;
foods::vegetables::potato::i_am_potato();
// vs
use foods::vegetables::potato::i_am_potato() as MyPotato;
MyPotato::i_am_potato();
```

Drastically reducing the convolution.

#### Accessibility

This has its pros and cons, cons being accessibility, pros being lack of accessibility. This allows for structure wrapping of non-generic utility functions. 

While file parsing might need to be accessable to every part of the program. Counting the amount of healthy vegetables may be needed by the garden module.

As higher up in main only needs the total quantity required, which is achieved through a series of wrapper methods.

An important note is struct fields are private by default.

``` Rust
pub struct StandardFood
{
  pub quanitiy:i32,
  secret_sause: String
}

impl StandardFood
{
  pub fn new(quanitiy:i32, secret_str: String) -> Self{
    StandardFood{quanitiy:quanitiy, secret_sause:secret_str}
  } // The instantiation method is public
  pub fn modify_sause(mut self, NewRecipe:String) // the mutator is also public beyond the scope
  {
    self.secret_sause = NewRecipe;
  }
  pub fn i_said_show_me_the_sause(self) //without using this function the sause is inaccessible
  {
    println!("{}",self.secret_sause);
  }
}//new_food.secret_str //But the variable is completely inaccessible, 

```

* Note I'll need to learn more about the rust compiler before truely understanding the application of this forced limitaiton.

Additionally its possible to extend the accessibility of a sub-module from a module with access to it.

``` Rust
  // in chapter_7.rs
mod foods;

pub use foods::vegetables::potato as MyPotato;
  // in main.rs
chapter_7::MyPotato::i_am_potato();
```
