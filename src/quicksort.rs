use rand::Rng;

fn aleatoriza(a: &mut [usize], p: isize, r: isize) -> isize {
    let mut rng = rand::rng();
        let i: usize = rng.random_range(p as usize..=r as usize);
    
    a.swap(r as usize, i as usize);
    particiona_aleatorizado(a, p, r)
}

pub fn quicksort(a: &mut [usize], p: isize, r: isize) {
    if p < r {
        let q = aleatoriza(a, p, r); 
        quicksort(a, p, q - 1);
        quicksort(a, q + 1, r);
    }
}

pub fn particiona_aleatorizado(a: &mut [usize], p: isize, r: isize) -> isize {
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
