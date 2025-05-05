use crate::quicksort::particiona_aleatorizado;                         

pub fn selecao_aleatorea(a: &mut [usize], p: isize,r: isize, i: isize) -> isize  {
    if p == r {
        return a[p as usize] as isize;
    }
    let q = particiona_aleatorizado(a, p, r);
    let k = q - p + 1;
    if i == k {
        return a[q as usize] as isize;
    }
    else if i < k {
        return selecao_aleatorea(a, p, q-1, i)
    }else {
        return selecao_aleatorea(a, q+1, r, i-k)
    }
}
