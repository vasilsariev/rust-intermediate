enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Cube<T, U, V> {
    height: T,
    width: U,
    length: V,
}

fn struct_example() {
    let cube1 = Cube {
        height: 1,
        width: 2,
        length: 3.5,
    };
}

pub fn lookup_datatype<T>(object: T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn mul_nums<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}
