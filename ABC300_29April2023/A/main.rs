use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize;n],
    }
    let sum: usize = a + b;
    for i in 0..n {
        if c[i] == sum {
            println!("{}", i + 1);
        }
    }
}
