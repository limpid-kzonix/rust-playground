use rand::{self, RngCore, rngs::ThreadRng};

fn main() {
    println!("Hello world!");
    let rnd_array = generate_rnd_array(10);
    println!("Array: {:?}", rnd_array);
}

fn generate_rnd_array(size: usize) -> Vec<u32> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut arr: Vec<u32> = Vec::with_capacity(size);
    for _ in 0..size {
        arr.push(rng.next_u32());
    }
    return arr;

}

    
