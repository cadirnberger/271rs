type Heap<T> = Vec<T>;

fn heapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    return h.push(gt)

}

fn reheapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    todo!();
}

fn vec_to_heap<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Heap<T> {
    todo!();
}

fn heap_to_vec<T>(mut h: Heap<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    todo!();
}

fn hsort<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    return heap_to_vec(vec_to_heap(xs, gt), gt);
}

fn main() {
    let xs: Vec<u64> = vec![2, 4, 6, 8, 5, 3, 7];
    fn f(x: &u64, y: &u64) -> bool {
        return x > y;
    }
    dbg!(&xs);
    let sorted: Vec<u64> = hsort(xs, f);
    dbg!(&sorted);
    return;
}
