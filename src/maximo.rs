pub fn maximo(a: &[usize], n: usize) -> usize {
    let mut max = a[0];
    for i in 0..n  {
        if max < a[i] {
            max = a[i];
        }
    }
    return max;
}


