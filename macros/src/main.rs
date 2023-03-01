/// This macro lets you add multiple items at once to a vector
#[macro_export]
macro_rules! push {
    ($target: expr, $($val: expr),+) => {
        $(
            $target.push($val);
        )+
    };
}

fn main() {
    let mut my_vec = Vec::new();
    push!(my_vec, 12, 14, 19, 254, 123);
    println!("Hello, world!");
}
