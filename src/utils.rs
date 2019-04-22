use std::io;
use std::io::Write;
use std::process::Command;

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
      -444
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
    /* 4 => {
      health = 70;
      dmg = 5;
      money = 1000000000;
    }*/
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

pub fn clear() {
  println!("\n\n\tPress enter to continue...");
  read();
  // clear terminal
  let output = Command::new("clear").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
  });
  println!("{}", String::from_utf8_lossy(&output.stdout));
}