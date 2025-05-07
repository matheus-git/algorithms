mod heap;
mod quicksort;
mod counting_sort;
mod minimo;
mod maximo;
mod selecao_aleatorea;
mod pilha;

use pilha::{pop, pilha_vazia};

fn main() {
    let mut arr = vec![8,4,5,2,9,3,6];
    pop(&mut arr);
    println!("{:?}", pilha_vazia(&arr));
}



