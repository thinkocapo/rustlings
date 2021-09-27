// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

fn main() {
    let mut vec = Vec::new();

    // should print the original 'vec' later, preserve it, don't create new vec1 yet
    // let vec0 = vec; //.clone();
    // let mut vec1 = fill_vec(vec0);

    // OR
    let vec0 = vec;
    vec = fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);

    vec.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
