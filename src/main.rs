use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();
    println!("Hello, world!, {}", i);
}