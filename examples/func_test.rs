
struct Test {
    pub func: fn(f64) -> f64,
}

fn exe(x: f64) -> f64 {
    x * x
}

fn main() {
    let test = Test {
        func: exe,
    };

    let res = (test.func)(2.0);

    println!("{}", res);
}
