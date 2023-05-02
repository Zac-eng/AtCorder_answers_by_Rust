use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut a: u64 = 2;
    let mut b: u64 = 3;
    let mut c: u64 = 5;
    let mut multi_count: u32 = 0;
    while a.pow(2) * b * c.pow(2) <= n {
        while a.pow(2) * b * c.pow(2) <= n {
            while a.pow(2) * b * c.pow(2) <= n {
                multi_count += 1;
                c = next_prime(c + 1);
            }
            b = next_prime(b + 1);
            c = next_prime(b + 1);
        }
        a = next_prime(a + 1);
        b = next_prime(a + 1);
        c = next_prime(b + 1);
    }
    println!("{}", multi_count);
}

fn next_prime(nb: u64) -> u64 {
    let mut nb: u64 = nb;
    if nb <= 2 {return 2}
    else if nb % 2 == 0 {nb += 1;}
    while judge_prime(nb) == false {
        nb += 2;
    }
    return nb
}

fn judge_prime(nb: u64) -> bool {
    let mut divisor: u64 = 3;
    if nb % 2 == 0 {
        return false
    }
    while divisor <= nb / divisor {
        if nb % divisor == 0 {
            return false
        }
        else {
            divisor += 2;
        }
    }
    return true
}