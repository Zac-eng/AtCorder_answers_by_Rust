use proconio::input;
use std::collections::HashMap;

const MODULO: u64 = 998244353;
const MOD_ONE_FIFTHS: u64 = 598946612;

fn main() {
    input! {
        n: u64,
    }
    let mut prob_map:HashMap<u64, u64> = HashMap::new();
    prob_map.insert(1, 1);
    prob_map.insert(2, MOD_ONE_FIFTHS);
    prob_map.insert(3, MOD_ONE_FIFTHS);
    prob_map.insert(5, MOD_ONE_FIFTHS);
    let ans: u64 = dp(n, &mut prob_map);
    println!("{}", ans);
}

fn dp(nb: u64, prob_map_ref: &mut HashMap<u64, u64>) -> u64 {
    match prob_map_ref.get(&nb) {
        Some(&probability) => return probability,
        _ => {
            let mut nb_prob: u64 = 0;
            if nb % 2 == 0 {nb_prob += mul_and_mod(dp(nb / 2, prob_map_ref));}
            if nb % 3 == 0 {nb_prob += mul_and_mod(dp(nb / 3, prob_map_ref));}
            if nb % 4 == 0 {nb_prob += mul_and_mod(dp(nb / 4, prob_map_ref));}
            if nb % 5 == 0 {nb_prob += mul_and_mod(dp(nb / 5, prob_map_ref));}
            if nb % 6 == 0 {nb_prob += mul_and_mod(dp(nb / 6, prob_map_ref));}
            nb_prob %= MODULO;
            prob_map_ref.insert(nb, nb_prob);
            return nb_prob
        }
    }
}

fn mul_and_mod(nb: u64) -> u64 {
    let mut ans: u64 = nb * MOD_ONE_FIFTHS;
    ans %= MODULO;
    ans
}
