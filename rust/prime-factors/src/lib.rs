struct Remainder {
    m: u64,
    counter: u64,
}

fn factor(n: u64, f: u64) -> Remainder {
    let mut m = n;
    let mut counter = 0;

    while m % f == 0 {
        m = m / f;
        counter += 1;
    }

    Remainder {
        m: m,
        counter: counter,
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    let mut remainder = factor(n, 2);
    let mut m = remainder.m;

    if remainder.counter > 0 {
        for _ in 0..remainder.counter {
            result.push(2);
        }
    }

    let mut f = 3;

    while m != 1 {
        remainder = factor(m, f);

        if remainder.counter > 0 {
            for _ in 0..remainder.counter {
                result.push(f);
            }
        }

        f += 2;
        m = remainder.m;
    }

    result
}

// fn main() {
//   println!("{}, {}", factor(12, 2).0, factor(12, 2).1);
// }
