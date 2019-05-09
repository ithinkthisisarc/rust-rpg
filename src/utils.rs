use std::io;
use std::io::Write;
use std::process::Command;
use rand::Rng;
use std::cmp::Ordering;

pub struct Player {
  pub name:   String,
  pub class:  i32,
  pub money:  i32,
  pub health: i32,
  pub dmg:    i32,
}

impl Player {
  pub fn gen_class(&mut self, class: i32) {
    match class {
      1 => {
        self.health = 20;
        self.dmg = 5;
        self.money = 0;
      },
      2 => {
        self.health = 10;
        self.dmg = 7;
        self.money = 50;
      },
      3 => {
        self.health = 7;
        self.dmg = 15;
        self.money = 0;
      },
      4 => {
        self.health = 70;
        self.dmg = 5;
        self.money = 1000000000;
      }
      _ => println!("'{}' is not a valid class!", class),
    };
  }
  pub fn shop(&mut self) {
    printf("Select something to purchase (1-2)\n 1: Upgraded health\n 2: Upgraded weapon\n> ");
    let opt = read();
    let opt: i32 = match opt.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please input a number!");
        return;
      }
    };
    match opt {
      1 => {
        if self.money > 49 {
          self.health += 3;
          println!("Health is now {}!", self.health);
        } else {
          println!("Not enough money...you need $50 to buy this!");
        }
      },
      2 => {
        if self.money > 49 {
          self.dmg += 3;
          println!("Damage is now {}!", self.dmg);
        } else {
          println!("Not enough money...you need $50 to buy this!");
        }
      },
      _ => println!("Option '{}' is invalid....", opt),
    };
  }

  pub fn inv(&mut self) {
    println!("NAME: {}\nCLASS: {}\nMONEY: {}\nHEALTH: {}\nDMG: {}", self.name, self.class, self.money, self.health, self.dmg);
  }

  pub fn fight(&mut self) -> i32{
    let mut bosshealth = 30;
    let mut plrhealth = self.health;
    let mut turn = 0;
    loop {
      clear();

      turn += 1;
      println!("TURN {}", turn);
      println!("Player health: {}\nBoss health: {}\n", plrhealth, bosshealth);
      let secret = rand::thread_rng().gen_range(1, 4);
      println!("Get ready for a fight!");
      println!("CHOOSE A DOOR");
      println!(" _____   _____   _____ ");
      println!("|     | |     | |     |");
      println!("|     | |     | |     |");
      println!("|  1  | |  2  | |  3  |");
      println!("|_____| |_____| |_____|\n");
      printf("> ");
      let i = readi();
      if i == -444 {
        continue;
      }
      match i.cmp(&secret) {
        Ordering::Less => println!("Wrong one! The boss was inside {}", secret),
        Ordering::Greater => println!("Wrong one! The boss was inside {}", secret),
        Ordering::Equal => {
          println!("You got it!\n");
          let plrdmg = rand::thread_rng().gen_range(self.dmg - 2, self.dmg);
          println!("You deal {} damage", plrdmg);
          bosshealth -= plrdmg;
        }
      };
      let bossdmg = rand::thread_rng().gen_range(7, 15);
      println!("The boss deals {} damage", bossdmg);
      plrhealth -= bossdmg;
      println!("WHERE WE'RE AT\nBOSS: {}\nPLAYER: {}", bosshealth, plrhealth);
      if plrhealth < 1 {
        println!("You died. Try harder next time...");
        clear();
        return 0;
      } else if bosshealth < 1 {
        println!("You win! You get $50!!!");
        clear();
        return 50;
      } else {
        continue;
      }
    }
  }

  pub fn game(&mut self, game: i32) {
    match game {
      0 => {
        println!("Playing dice");
        let num = rand::thread_rng().gen_range(1, 6);
        let mut tries = 2;
        while tries != 0 {
          printf("Enter your bet: ");
          let guess = readi();
          if guess == num {
            println!("You got it right! You earned $30");
            self.money += 30;
            return;
          } else if guess > num {
            println!("Too big...");
          } else if guess < num {
            println!("Too small...");
          } else {
            println!("????????????");
          }
          tries = tries - 1;
        }
        println!("The number was '{}'", num);
        self.money += 0;
      },
      1 => {
        println!("Playing guess");
        let num = rand::thread_rng().gen_range(1, 10);
        let mut tries = 3;
        while tries != 0 {
          printf("Enter your guess: ");
          let guess = readi();
          if guess == num {
            println!("You got it right! You earned $10");
            self.money += 0;
            return;
          } else if guess > num {
            println!("Too big...");
          } else if guess < num {
            println!("Too small...");
          } else {
            println!("????????????");
          }
          tries = tries - 1;
        }
        println!("The number was '{}'", num);
        self.money += 0;
      },
      _ => {
        println!("Weird...a bad number was generated. Expected 0 or 1, got {}.", game);
        self.money += 0;
      }
    };
  }
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

pub fn clear() {
  println!("\n\n\tPress enter to continue...");
  read();
  // clear terminal
  let output = Command::new("clear").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
  });
  println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn clear_without_message() {
  let output = Command::new("clear").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
  });
  println!("{}", String::from_utf8_lossy(&output.stdout));
}