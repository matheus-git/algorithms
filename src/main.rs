mod heap;
mod quicksort;
mod counting_sort;

use quicksort::particiona_aleatorizado;                         

fn maximo(a: &[usize], n: usize) -> usize {
    let mut max = a[0];
    for i in 0..n  {
        if max < a[i] {
            max = a[i];
        }
    }
    return max;
}

fn minimo(a: &[usize], n: usize) -> usize {
    let mut min = a[0];
    for i in 0..n  {
        if min > a[i] {
            min = a[i];
        }
    }
    return min;
}

fn selecao_aleatorea(a: &mut [usize], p: isize,r: isize, i: isize) -> isize  {
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

fn main() {
    let mut arr = vec![8,4,5,2,9,3,6];
    let min = selecao_aleatorea(&mut arr, 0,6, 2);
    println!("{:?}", min);
}
