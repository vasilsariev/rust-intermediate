use std::collections::HashMap;

#[macro_export]
macro_rules! make_vec {
    ( $( $x:expr),* ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

#[macro_export]
macro_rules! make_map {
    ($( $key:expr, $value:expr),* ) => {{
        let mut temp_map = HashMap::new();
        $(
            temp_map.insert($key, $value);
        )*
        temp_map
    }};
}

pub fn use_macro() {
    let vec = make_vec!['1', '2'];
    let map = make_map![("one", 111), ("two", 122)];
    println!("Values in the map{:#?}", map);
}
