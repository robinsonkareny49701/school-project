use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, 7);
    println!("{}", roll);
}
