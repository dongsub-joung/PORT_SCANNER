use gcd::Gcd;
use rand::Rng;
use std::convert::Tryinto;

pub struct RangeIterator{
    active: bool,
    normalized_end: u32,
    normalized_first_pick: u32,
    normalized_pick: u32,
    actual_start: u32,
    step: u32,
}

impl RangeInterator {
    pub fn new(start: u32, end: u32) -> Self{
        let normalized_end= end- start;
        let step= pick_random_coprime(normalized_end);
        
        let mut rng= rand::thread_rng();
        let normalized_frist_pick= rng.gen_range(0, normalized_end);

        Self{
            active: true,
            normalized_end,
            step,
            normalized_first_pick,
            normalized_pick: normalized_first_pick,
            actual_start: start,
        }
    }
}

impl Iterator for RangeIterator {
    type Item= u16;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.active{
            return  None;
        }

        let current_pick= self.normalized_first_pick;
        let next_pick= (cure)
    }
}