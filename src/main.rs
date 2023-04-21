use rand::{thread_rng, Rng};


fn main() {
    let mut rng = thread_rng();
    let num = rng.gen_range(49152..65535);
    println!("{}", num);
}
