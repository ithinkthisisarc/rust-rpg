mod utils;
mod menuopts;

use crate::utils::*;
use crate::menuopts::*;
use rand::Rng;

fn main() {
  printf("Enter your name: ");
  let name = read();
  printf("Choose a class (1-3)\n 1: Wizard\n 2: Assassin\n 3: Warrior\n> ");
  let class = read();
  let class: i32 = match class.parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Please input a number!");
      return;
    }
  };
  let mut player = gen_class(name, class);
  loop {
    printf("What would you like to do? (1-4)\n 1: Visit the shop\n 2: Check inventory\n 3: Fight\n 4: Play a game\n> ");
    let choice = read();
    let choice: i32 = match choice.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please input a number!");
        continue;
      }
    };
    match choice {
      1 => shop(&mut player),
      2 => inv(&mut player),
      3 => player.money += fight(&mut player),
      4 => {
        let i: i32 = rand::thread_rng().gen_range(0,2);
        let reward: i32 = match i {
          0 => dice(),
          1 => guess(),
          _ => {
            println!("Bad number '{}' generated...that's weird", i);
            0
          },
        };
        player.money += reward;
      },
      _ => println!("Option '{}' is invalid....", choice),
    };
  }
}
