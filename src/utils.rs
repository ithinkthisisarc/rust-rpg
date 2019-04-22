use std::io;
use std::io::Write;

pub struct Player {
  pub name:   String,
  pub class:  i32,
  pub money:  i32,
  pub health: i32,
  pub dmg:    i32,
}

pub fn printf(s: &str) {
  print!("{}", s);
  io::stdout().flush().unwrap();
}

pub fn read() -> String {
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Failed to read line...");
  let mut inp: &str = inp.as_ref();
  inp = inp.trim();
  let inp: String = inp.to_string();
  return inp;
}

pub fn readi() -> i32 {
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Failed to read line...");
  let mut inp: &str = inp.as_ref();
  inp = inp.trim();
  let inp: String = inp.to_string();
  let inp: i32 = match inp.parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Please enter a number!");
      0
    }
  };
  return inp;
}

pub fn gen_class(name: String, class: i32) -> Player {
  let mut health: i32 = 0;
  let mut money: i32 = 0;
  let mut dmg: i32 = 0;
  match class {
    1 => {
      health = 20;
      dmg = 5;
      money = 0;
    },
    2 => {
      health = 10;
      dmg = 7;
      money = 50;
    },
    3 => {
      health = 7;
      dmg = 15;
      money = 0;
    },
    _ => println!("'{}' is not a valid class!", class),
  };

  let player = Player {
    name,
    class,
    money,
    health,
    dmg,
  };
  return player;
}
