pub fn clog2(n: usize) -> Option<usize> {
    let casted = n as u64;
    if casted == 0 {
        None
    } else {
        Some((64 - (casted - 1).leading_zeros()) as usize)
    }
}

pub fn sel(n: usize, size: usize) -> String {
    if size == 1 {
        format!("")
    } else {
        format!("[{}]", n)
    }
}

pub fn range(begin: usize, end: usize) -> String {
    if begin == 0 {
        format!("")
    } else {
        format!("[{}:{}]", begin - 1, end)
    }
}
