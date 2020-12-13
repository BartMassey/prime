//! Miller-Rabin probabilistic test for composite numbers
//! as described in Corman/Leiserson/Rivest.
//!
//! Translated from a 1999 [Nickle](http://nickle.org) example.

use argh::FromArgs;
use num_bigint::*;
use rand::prelude::*;

#[derive(FromArgs)]
/// primality test
struct Args {
    #[argh(option, short = 'd', default = "1024")]
    /// composite failure probability 1/2**d
    d: u64,
    #[argh(positional)]
    /// number to test
    n: BigUint,
}

struct Witness {
    pow: BigUint,
    wit: BigUint,
}

/// Modified version of bigpowmod() from numbers.5c.
/// Computes core of Miller-Rabin test as suggested by
/// Cormen/Leiserson/Rivest.
fn witnessexp(b: &BigUint, e: &BigUint, m: &BigUint) -> Witness {
    if *e == BigUint::from(0u8) {
        return Witness {
            pow: BigUint::from(0u8),
            wit: BigUint::from(1u8),
        };
    }
    if *e == BigUint::from(1u8) {
        return Witness {
            pow: b % m,
            wit: BigUint::from(0u8),
        };
    }
    let ehalf = e / BigUint::from(2u8);
    let mut tmp = witnessexp(b, &ehalf, m);
    if tmp.wit != BigUint::from(0u8) {
        return tmp;
    }
    let t: BigUint = BigUint::pow(&tmp.pow, 2) % m;
    if t == BigUint::from(1u8)
        && tmp.pow != BigUint::from(1u8)
        && &tmp.pow + BigUint::from(1u8) != *m
    {
        tmp.wit = tmp.pow;
        tmp.pow = t;
        return tmp;
    }
    if e % BigUint::from(2u8) == BigUint::from(0u8) {
        tmp.pow = t;
    } else {
        tmp.pow = (t * b) % m;
    }
    tmp
}

/// Rest of Miller-Rabin test.
fn witness(a: &BigUint, n: &BigUint) -> bool {
    let ndown = n - &BigUint::from(1u8);
    let we = witnessexp(a, &ndown, n);
    we.wit != BigUint::from(0u8) || we.pow != BigUint::from(1u8)
}

/// Returns false for all prime n, and true for most
/// composite n. Failure probability is 1/2**d.
fn composite(n: &BigUint, d: u64) -> bool {
    let mut rng = rand::thread_rng();
    let primes = [2u8, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for p in &primes {
        let p = BigUint::from(*p);
        if p > *n {
            return true;
        }
        if *n == p {
            return false;
        }
        if n % p == BigUint::from(0u8) {
            return true;
        }
    }
    for _ in 0..d {
        let a = rng.gen_range(BigUint::from(1u8), n);
        if witness(&a, &n) {
            return true;
        }
    }
    false
}

fn main() {
    let args: Args = argh::from_env();
    println!("{}", !composite(&args.n, args.d));
}
