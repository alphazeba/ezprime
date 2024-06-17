
mod prime_digestor;
mod input;

use prime_digestor::PrimeDigestor;
use input::Input;

fn main() {
    let args = Input::get();
    let mut digestor = PrimeDigestor::new();
    let result = digestor.is_prime(args.number);
    println!("{}",result.to_string());
    if args.metrics {
        println!("\n\nmetrics:\n{}", digestor.metrics_to_string())
    }
}
