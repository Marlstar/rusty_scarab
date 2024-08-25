pub type ModVersion = [usize;4];

pub fn newer_version_than(lhs: &ModVersion, other: &ModVersion) -> bool {
    for i in 0..=3 {
        if lhs[i] > other[i] { return true }
    }
    return false;
}

pub fn older_version_than(lhs: &ModVersion, other: &ModVersion) -> bool {
    for i in 0..=3 {
        if lhs[i] < other[i] { return true }
    }
    return false;
}
