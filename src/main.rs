use ndarray::Array;
use ndarray_npy::write_npy;
use rand::prelude::*;

const N: usize = 1_000_00 ;//000_000;
const WALKERS: usize = 20;
const DIM1: usize = 1920;
const DIM2: usize = 1080;

fn main() {
    let mut rng = thread_rng();
    let mut world: Array<i64, _> = Array::zeros((DIM1, DIM2));
    let mut pos = (1..=WALKERS)
        .map(|_| {
            (
                (1..DIM1).choose(&mut rng).unwrap(),
                (1..DIM2).choose(&mut rng).unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();
    for _ in 1..=N {
        pos = pos
            .into_iter()
            .map(|(x, y)| {
                let xold = x as i64;
                let yold = y as i64;
                loop {
                    let dx: i64 = (-5..=5).choose(&mut rng).unwrap();
                    let dy: i64 = (-5..=5).choose(&mut rng).unwrap();
                    let xnew = xold + dx;
                    let ynew = yold + dy;
                    if xnew >= 0 && xnew < DIM1 as i64 && ynew >= 0 && ynew < DIM2 as i64 {
                        break (xnew as usize, ynew as usize);
                    }
                }
            })
            .collect::<Vec<(usize, usize)>>();
        pos.iter().for_each(|(x, y)| world[[*x, *y]] += thread_rng().gen_range(1..100));
    }
    let filename = format!("scrsaver_arr{}.npy", thread_rng().gen_range(1..=1000));
    write_npy(filename, &world).unwrap();
}
