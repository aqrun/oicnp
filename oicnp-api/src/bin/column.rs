use oicnp_derives::{Column};

#[derive(Column)]
enum Robot {
    #[oic(comment = "这是Apple")]
    Apple,
    Banana,
}

fn main() {
    println!("{}", Robot::Apple.comment());
    println!("{}", Robot::Banana.comment());
}