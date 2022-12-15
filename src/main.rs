use tch::{Tensor, jit};


fn my_test() {
    let t = Tensor::of_slice(&[3, 2, 1, 2]);
    t.print();
}


fn main() {
    my_test();
}
