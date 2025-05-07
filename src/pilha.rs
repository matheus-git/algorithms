pub fn pilha_vazia(s: &Vec<usize>) -> bool {
    s.is_empty()    
}

pub fn push(s: &mut Vec<usize>, x: usize){
    s.push(x)
}

pub fn pop(s: &mut Vec<usize>){
    if s.pop().is_none() {
        panic!("Pilha vazia!");
    }
}

pub fn topo(s: &Vec<usize>) -> usize {
    if let Some(topo) = s.last() {
        *topo
    }else{
        panic!();
    }
}

