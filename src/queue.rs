use std::collections::VecDeque;

fn enqueue(s: &mut VecDeque<usize>, x: usize) {
   s.push_back(x); 
}

fn dequeue(s: &mut VecDeque<usize>) -> usize {
   match s.pop_front() {
        Some(x) => x,
        None => panic!()
   }
}


