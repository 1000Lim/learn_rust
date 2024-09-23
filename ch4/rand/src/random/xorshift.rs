pub fn rand(seed: &mut u32) -> u32 {
    let x = *seed;
    *seed = x ^ (x << 13);
    *seed = *seed ^ (*seed >> 17);
    *seed = *seed ^ (x << 5);
    *seed
}