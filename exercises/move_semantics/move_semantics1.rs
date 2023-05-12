// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.



fn main() {
    let vec0 = Vec::new();

    // // original:
    // let mut vec1 = fill_vec(vec0); // when vec0 is moved, rust does a shallow copy of the pointer to the heap
    // // alternative 1:
    // let mut vec1 = fill_vec(Vec::new()); // skipping the move
    // // alternative 2:
    let mut vec1 = fill_vec(vec0.clone()); // when clone() is called, rust does a deep copy of the heap, so vec0 is still valid

    let mut vec2 = fill_vec(vec0); 
    
    // // also, when fill_vec takes ownership of vec0 (in the original), vec0 is no longer valid, so we can create it again:
    // let vec0 = Vec::new();
    // let mut vec2 = fill_vec(vec0);


    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
