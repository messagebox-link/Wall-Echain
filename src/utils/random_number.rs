use log::debug;
use rand::{thread_rng, Rng};

pub fn random_number(max: usize) -> usize {
    debug!("🔧 Random maximum {:?}", max);
    let mut rng = thread_rng();
    rng.gen_range(0..=(max as u128 - 1)) as usize
}