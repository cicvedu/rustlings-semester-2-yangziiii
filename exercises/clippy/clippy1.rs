// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let pi = std::f32::consts::PI; // 使用常量 PI 替代手动赋值
    let radius: f32 = 5.00;

    let area = pi * radius.powf(2.0); // 使用 powi 方法来计算平方

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}

