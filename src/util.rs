pub fn clog2(n: usize) -> Option<usize> {
    let casted = n as u64;
    if casted == 0 {
        None
    } else {
        Some((64 - (casted - 1).leading_zeros()) as usize)
    }
}
