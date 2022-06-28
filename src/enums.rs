enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  match m {
    Movement::Up => println!("Avatar moving Up"),
    Movement::Down => println!("Avatar moving Down"),
    Movement::Left => println!("Avatar moving Left"),
    Movement::Right => println!("Avatar moving Right"),
  }
}

pub fn run() {
  let movement1 = Movement::Left;
  let movement2 = Movement::Up;
  let movement3 = Movement::Right;
  let movement4 = Movement::Down;

  move_avatar(movement1);
  move_avatar(movement2);
  move_avatar(movement3);
  move_avatar(movement4);
}
