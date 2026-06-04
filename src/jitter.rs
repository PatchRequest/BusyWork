use rand::Rng;

pub fn apply(base: usize, rng: &mut impl Rng) -> usize {
    let factor: f64 = rng.gen_range(0.7..=1.3);
    (base as f64 * factor).round().max(1.0) as usize
}
