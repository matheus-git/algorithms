use algorithms::{minimo, maximo, heap, quicksort, counting_sort, pilha, rod_cutting};

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

#[test]
fn test_rod_cutting() {
    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
    let length = 8;

    assert_eq!(rod_cutting::rod_cutting(&prices, length), 22);
}

