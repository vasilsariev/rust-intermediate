pub fn example_1(x: i32, y: i32) -> i32 {
    let multiply = |x| {
        println!("Multiplying {} by itself", x);
        x * x
    };
    if x > y { multiply(x) } else { multiply(y) }
}
