mod utils;

use crate::utils::*;
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
  let mut player = Player{
    name,
    class,
    money: 0,
    health: 0,
    dmg: 0,
  };
  player.gen_class(class);
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
      1 => player.shop(),
      2 => player.inv(),
      3 => player.money += player.fight(),
      4 => {
        let i: i32 = rand::thread_rng().gen_range(0,2);
        player.game(i);
      },
      _ => println!("Option '{}' is invalid....", choice),
    };
    if choice != 3 {
      clear();
    } else {
      clear_without_message();
    }
  }
}
