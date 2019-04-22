use crate::utils::*;
use rand::Rng;
use std::cmp::Ordering;

pub fn shop(player: &mut Player) {
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
      if player.money > 49 {
        player.health += 3;
        println!("Health is now {}!", player.health);
      } else {
        println!("Not enough money...you need $50 to buy this!");
      }
    },
    2 => {
      if player.money > 49 {
        player.dmg += 3;
        println!("Damage is now {}!", player.dmg);
      } else {
        println!("Not enough money...you need $50 to buy this!");
      }
    },
    _ => println!("Option '{}' is invalid....", opt),
  };
}

pub fn inv(player: &mut Player) {
  println!("NAME: {}\nCLASS: {}\nMONEY: {}\nHEALTH: {}\nDMG: {}", player.name, player.class, player.money, player.health, player.dmg);
}

pub fn fight(player: &mut Player) -> i32{
  let mut bosshealth = 30;
  let mut plrhealth = player.health;
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
        let plrdmg = rand::thread_rng().gen_range(player.dmg - 2, player.dmg);
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

pub fn dice() -> i32 {
  println!("Playing dice");
  let num = rand::thread_rng().gen_range(1, 6);
  let mut tries = 2;
  while tries != 0 {
    printf("Enter your bet: ");
    let guess = readi();
    if guess == num {
      println!("You got it right! You earned $30");
      return 30;
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
  return 0;
}

pub fn guess() -> i32{
  let num = rand::thread_rng().gen_range(1, 10);
  let mut tries = 3;
  while tries != 0 {
    printf("Enter your guess: ");
    let guess = readi();
    if guess == num {
      println!("You got it right! You earned $10");
      return 10;
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
  return 0;
}
