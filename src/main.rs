use rand_mt::{Mt19937GenRand64, Mt64};
use std::cmp::Ordering;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::iter::Iterator;
 

#[inline(always)]
fn random_number(generator: &mut Mt19937GenRand64, max: u64) -> usize {
    (generator.next_u64() % max) as usize
}

fn experiment(box_count: &u64, gen: &mut Mt19937GenRand64, a: &mut u64,
              b: &mut u64, c: &mut u64, d: &mut u64, e: &mut u64)
{
    let mut boxes: Vec<u64> = vec![0; *box_count as usize];
    let mut ball_count: u64 = 0;

    let mut a_worked: bool = false;
    let mut b_worked: bool = false;
    let mut c_worked: bool = false;
    let mut d_worked: bool = false;
    let mut e_worked: bool = false;

    let mut a_result: u64 = 0;
    let mut b_result: u64 = 0;
    let mut c_result: u64 = 0;
    let mut d_result: u64 = 0;
    let mut e_result: u64 = 0;

    loop {
        ball_count += 1;

        let index = random_number(gen, *box_count);
        boxes[index] += 1;

        if !a_worked && a_check(&boxes) {
            a_worked = true;
            a_result = ball_count;
        }

        if !b_worked && b_check(&ball_count, &box_count) {
            b_worked = true;
            b_result = b_count(&boxes);
        }

        if !c_worked && c_check(&boxes) {
            c_worked = true;
            c_result = ball_count;
        }

        if !d_worked && d_check(&boxes) {
            d_worked = true;
            d_result = ball_count;
        }

        if d_worked && c_worked {
            e_worked = true;
            e_result = d_result - c_result;
        }

        if a_worked && b_worked && c_worked && d_worked && e_worked {

            for cur_box in boxes.iter() {
                print!("{}; ", cur_box);
            }
            println!();

            *a = a_result;
            *b = b_result;
            *c = c_result;
            *d = d_result;
            *e = e_result;

            break;
        }
    }
}

fn a_check(boxes: &Vec<u64>) -> bool {
    for n in boxes.iter() {
        match n.cmp(&1) {
            Ordering::Greater => return true,
            _ => {}
        }
    }
    return false;
}

#[inline(always)]
fn b_check(ball_count: &u64, boxes_count: &u64) -> bool {
    return match boxes_count.cmp(ball_count) {
        Ordering::Equal => true,
        _ => false,
    };
}

fn b_count(boxes: &Vec<u64>) -> u64 {
    let mut empty_boxes_count: u64 = 0;
    for n in boxes.iter() {
        match n.cmp(&0) {
            Ordering::Equal => empty_boxes_count += 1,
            _ => {}
        }
    }
    empty_boxes_count
}

fn c_check(boxes: &Vec<u64>) -> bool {
    for n in boxes.iter() {
        match n.cmp(&1) {
            Ordering::Less => return false,
            _ => {}
        }
    }
    true
}

fn d_check(boxes: &Vec<u64>) -> bool {
    for n in boxes.iter() {
        match n.cmp(&2) {
            Ordering::Less => return false,
            _ => {}
        }
    }
    true
}

fn main() {
    let repeat_count: usize = 51;
    let n_list: Vec<u64> = (1..101).map(|x| x * 1000).collect::<Vec<u64>>();

    let mut a: [[u64; 1000]; 50] = [[0; 1000]; 50];
    let mut b: [[u64; 1000]; 50] = [[0; 1000]; 50];
    let mut c: [[u64; 1000]; 50] = [[0; 1000]; 50];
    let mut d: [[u64; 1000]; 50] = [[0; 1000]; 50];
    let mut e: [[u64; 1000]; 50] = [[0; 1000]; 50];

    let result_directory = Path::new("/home/romka/");
    let result_filename = result_directory.join("result");
    let mut file = File::create(result_filename).unwrap();

    for i in 1..repeat_count {
        // Nieudana próba zrobienia wielowątkowości podczas obliczenia niezależnych powtórzeń
        thread_runner(&i, &n_list, &mut a[i], &mut b[i], &mut c[i], &mut d[i], &mut e[i], &mut file);
    }
}

fn thread_runner(repeat: &usize, n_list: &Vec<u64>, a: &mut [u64; 1000], b: &mut [u64; 1000],
                 c: &mut [u64; 1000], d: &mut [u64; 1000], e: &mut [u64; 1000], file: &mut File) {
    let mut gen = Mt64::new(rand::random::<u64>());

    for (i, &box_count) in n_list.iter().enumerate() {
        experiment(&box_count, &mut gen, &mut a[i], &mut b[i], &mut c[i],
                   &mut d[i], &mut e[i]);
        println!("{};{};{};{};{};{};{}", repeat, box_count, a[i], b[i], c[i], d[i], e[i]);

        writeln!(file, "{};{};{};{};{};{};{}", repeat, box_count, a[i], b[i], c[i], d[i], e[i]).expect("TODO: panic message");
    }
}

