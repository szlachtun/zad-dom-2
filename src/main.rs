use rand_mt::{Mt, Mt19937GenRand32};
use std::cmp::Ordering;

static M: u32 = 4;
static N: u32 = 8;
static K: u32 = 50;

enum State {
    Continue,
    Break,
}

#[inline]
fn random_number(generator: &mut Mt19937GenRand32, max: &u32) -> u32 {
    generator.next_u32() % max
}

fn experiment(i: &u32, rng: &mut Mt19937GenRand32, a: &mut Vec<u32>, b: &mut Vec<u32>,
              c: &mut Vec<u32>, d: &mut Vec<u32>, e: &mut Vec<u32>) {
    let mut boxes: Vec<u32> = vec![0; M as usize];

    while experiment_check(&boxes) {
        let index = random_number(rng, &M) as usize;
        boxes[index] += 1;
        // println!("again");
    }
    // println!("ok");

}

fn experiment_check(boxes: &Vec<u32>) -> bool {
    for n in boxes.iter() {
        match n.cmp(&2) {
            Ordering::Less => return true,
            _ => {}
        }
    }
    return false;
}

fn main() {
    let mut rng = Mt::new(rand::random::<u32>());
    //let mut rng = Mt::new_unseeded();  // WARNING: DEFAULT RANDOM SEED

    let mut a = vec![0; K as usize];
    let mut b = vec![0; K as usize];
    let mut c = vec![0; K as usize];
    let mut d = vec![0; K as usize];
    let mut e = vec![0; K as usize];

    for i in 0..K {
        experiment(&i, &mut rng, &mut a, &mut b, &mut c, &mut d, &mut e);
    }

    println!("{}", random_number(&mut rng, &M));
}
