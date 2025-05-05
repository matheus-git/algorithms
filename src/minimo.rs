fn minimo(a: &[usize], n: usize) -> usize {
    let mut min = a[0];
    for i in 0..n  {
        if min > a[i] {
            min = a[i];
        }
    }
    return min;
}
