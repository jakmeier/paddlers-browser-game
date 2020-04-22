/// A simple linear congruential generator
pub struct Lcg {
    pub val: u64,
}
impl Lcg {
    // MMIX LCG
    const A: u64 = 6364136223846793005u64;
    const C: u64 = 1442695040888963407u64;
    pub fn new(seed: u64) -> Self {
        Lcg { val: seed }
    }
    /// Returns a pseudo-random value in the specified range, inclusive start and exclusive end
    pub fn next_in_range(&mut self, start: u64, end: u64) -> u64 {
        let i = self.next().unwrap();
        let m = end - start;
        start + i % m
    }
}
impl Iterator for Lcg {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.val = Self::A.wrapping_mul(self.val).wrapping_add(Self::C);
        Some(self.val)
    }
}
