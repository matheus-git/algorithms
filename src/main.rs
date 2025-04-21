use std::usize;

fn pai(i: usize) -> usize{
    i/2
}

fn esquerda(i: usize) -> usize {
    2 * i + 1 // Índice do filho esquerdo
}

fn direita(i: usize) -> usize {
    2 * i + 2  // Índice do filho direito
}


fn maximiza_heap(a: &mut [usize], i: usize) {
    let l = esquerda(i);
    let r = direita(i);
    let mut maior = i;

    if l < a.len() && a[l] > a[maior] {
        maior = l;
    }

    if r < a.len() && a[r] > a[maior] {
        maior = r;
    }

    if maior != i {
        a.swap(i, maior);
        maximiza_heap(a, maior);
    }
}

fn constroi_max_heap(a: &mut [usize]) {
    let n = a.len();
    for i in (0..n / 2).rev() {
        maximiza_heap(a, i);
    }
}

fn heapsort(a: &mut [usize]) {
    constroi_max_heap(a);
    for i in (1..a.len()).rev() {
        a.swap(0, i);
        maximiza_heap(&mut a[0..i], 0);
    }
}

fn main() {
    let mut a = vec![42, 17, 93, 8, 65, 58, 21, 74, 39, 88, 2, 30, 19, 50, 77, 12, 6, 91, 33, 26, 61, 85, 4, 47, 69, 10, 36, 99, 73, 55]
;
    constroi_max_heap(&mut a);
    println!("{:?}", a);
    heapsort(&mut a);
    println!("{:?}", a);
}
