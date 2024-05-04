use std::char::MAX;

use curv::{self, BigInt, arithmetic::{Converter, Modulo, Samplable}};

const MAXDEGREE: u8 = 32;

struct Poly {
    deg: u64,
    coef: [BigInt; MAXDEGREE],
}


