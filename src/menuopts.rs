use crate::utils::*;

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

pub fn fight() {

}

pub fn dice() -> i32 {
  let num = 5;
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
  let num = 7;
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
