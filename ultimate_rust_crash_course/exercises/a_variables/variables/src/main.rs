const STARTIN_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
  // part 1
  let mut missiles: i32 = 8;
  let mut ready: i32 = 2;

  println!("Firing {} of my {} missiles...", ready, missiles);

  // part 2
  missiles = missiles - ready;
  println!("{} missiles left", missiles);

  missiles = STARTIN_MISSILES;
  ready = READY_AMOUNT;

  println!("now with contants... ready: {} - missiles: {}", ready, missiles);

  // extra stuff

  // let (mut new_missiles, mut new_ready): (i32, i32) = (STARTIN_MISSILES, READY_AMOUNT);
  // results in -> variable does not need to be mutable

  println!("{} missiles left", missiles - ready);

  // let i_will_not_be_used = 1;
  // results in -> unused variable: `i_will_not_be_used`

  // READY_AMOUNT = 1;
  // results in -> invalid left-hand side of assignment
}
