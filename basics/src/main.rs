use std::str::FromStr;

fn main() {
    let mut ns: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        let i = u64::from_str(&arg);
        let r = i.expect("Parser error on numeric argument");
        ns.push(r);
    }

    if ns.is_empty() {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    // foldl1 over args
    let mut r = ns[0]; // there's at least one argument
    for p in &ns[1..] {
        r = gcd(r, *p);
    }

    println!("The GCD of {:?} is {}", ns, r);

    let s = str_games("I see the eigenvalue in thine eye");
    println!("{}", s);

    let b = array_stuff_2();
    for _ in 2 .. 10 {
        println!("{:?}", b[0]);
    }

    let v = vec_stuff();
    for i in 0 .. v.len() {
        println!("{:?}", v[i]);
    }
    println!("PRODUCT = {}", v.iter().fold(1, |a,b| a*b));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 77 * 11 * 13 * 19), 3 * 11);
}

#[allow(dead_code)]
// allocate a new vector and poke some values in
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10i16);
    v.push(20);
    v
}

fn str_games(t: &str) -> String {
    let (x, _xs) = str::split_at(t,21);
    let t = (12, "eggs"); // stack allocated. unboxed
    let _b = Box::new(t);
    x.to_string()
}

#[test]
fn array_stuff() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Anthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
}



fn array_stuff_2() -> Box<[bool]> {
    const A_LIM: usize = 10000;
    let mut sieve = [true; A_LIM];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < A_LIM {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    Box::new(sieve)
}

fn vec_stuff() -> Vec<i32> {
    let v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v
}
