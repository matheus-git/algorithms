fn quicksort(a: &mut [usize], p: isize, r: isize) {
    if p < r {
        let q = particiona(a, p, r);
        quicksort(a, p, q - 1);
        quicksort(a, q + 1, r);
    }
}

fn particiona(a: &mut [usize], p: isize, r: isize) -> isize {
    let x = a[r as usize];
    let mut i = p - 1;
    for j in p..r {
        if a[j as usize] <= x {
            i += 1;
            a.swap(i as usize, j as usize);
        }
    }
    a.swap((i + 1) as usize, r as usize);
    i + 1
}
