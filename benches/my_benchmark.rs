use criterion::{criterion_group, criterion_main, Criterion};

fn bench_dummy(c: &mut Criterion) {
    c.bench_function("noop", |b| b.iter(|| {}));
}

criterion_group!(benches, bench_dummy);
criterion_main!(benches);

#[cfg(test)]
mod tests {

    #[test]
    fn test_minimo() {
        let a = vec![3, 5, 2, 1, 7, 8, 4];
        assert_eq!(minimo::minimo(&a, 7), 1);
    }

    #[test]
    fn test_maximo() {
        let a = vec![3, 5, 2, 1, 7, 8, 4];
        assert_eq!(maximo::maximo(&a, 7), 8);
    }

    #[test]
    fn test_heapsort() {
        let mut a = vec![3, 5, 2, 1, 7, 8, 4];
        heap::heapsort(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn test_quicksort() {
        let mut a = vec![3, 5, 2, 1, 7, 8, 4];
        quicksort::quicksort(&mut a, 0, 6);
        assert_eq!(a, vec![1, 2, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn test_counting_sort() {
        let mut a = vec![3, 5, 2, 1, 7, 8, 4];
        counting_sort::counting_sort(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn test_pilha() {
        assert!(pilha::pilha_vazia(&vec![]));
        let mut a = vec![3, 5, 2, 1, 7, 8, 4];
        pilha::push(&mut a, 10);
        assert_eq!(a[a.len() - 1], 10);
        pilha::pop(&mut a);
        assert_eq!(pilha::topo(&a), 4);
    }
}

