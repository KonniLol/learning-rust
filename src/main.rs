fn main() {
    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;

    let z = x as i32 / y;
    println!("{}", z);
}
