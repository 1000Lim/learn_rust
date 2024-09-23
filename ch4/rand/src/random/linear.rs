use std::num::Wrapping;

pub fn rand(seed: &mut u32) -> u32 {
    let (a, c) = (134775813u32, 1234u32);
    *seed = (Wrapping(a) * Wrapping(*seed) + Wrapping(c)).0;
    *seed
}

