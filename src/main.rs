use rand_mt::{Mt, Mt19937GenRand32};

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

fn experiment(i: &u32, a: &mut Vec<u32>, b: &mut Vec<u32>, c: &mut Vec<u32>,
              d: &mut Vec<u32>, e: &mut Vec<u32>) {
    let mut boxes: Vec<u32> = vec![0; M as usize];




}

fn stop_experiment_case(boxes: &Vec<u32>) -> State {
    for n in boxes.iter() {
        if *n != 2u32 {
            return State::Continue;
        }
    }
    return State::Break;
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
        experiment(&i, &mut a, &mut b, &mut c, &mut d, &mut e);
    }

    println!("{}", random_number(&mut rng, &M));
}
