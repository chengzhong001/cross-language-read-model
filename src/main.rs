use tch::{jit, Tensor};

// fn my_test() {
//     let t = Tensor::of_slice(&[3, 2, 1, 2]);
//     t.print();
// }

#[warn(dead_code)]
fn run_infer() {
    // code-1
    let image_value = Tensor::randn(&[1, 1, 8, 8], tch::kind::FLOAT_CPU);

    // code-2
    let model = jit::CModule::load(
        "/Users/azhong/Documents/Project/cross-language-read-model/model/digit.jit",
    )
    .unwrap();
    let ouput = model.forward_ts(&[image_value]).unwrap();
    ouput.print();
}

fn main() {
    // my_test();
    run_infer();
    // println!("Hello, world!");
}
