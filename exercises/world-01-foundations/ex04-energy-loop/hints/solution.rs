use std::ops::RangeInclusive;

pub fn total_harvest(cycles: RangeInclusive<u32>) -> u32 {
    cycles.sum()
}

pub fn mission_harvest_report(missions: &[RangeInclusive<u32>]) -> Vec<u32> {
    missions.iter().cloned().map(total_harvest).collect()
}
