use rand::prelude::*;
use std::{cell::RefCell, rc::Rc, time::Instant};

pub const BIG_STRUCT_SIZE: usize = 16 * 1024;
pub const NUM_ENRICHERS: usize = 100;

pub const TRANSLATION: f64 = 0.;
pub const BASE: f64 = 16.;
pub const ITERATIONS: u64 = 10;

pub struct BigStruct {
    input: [f64; BIG_STRUCT_SIZE],
    out: [f64; BIG_STRUCT_SIZE],
    time: Instant,
}

impl BigStruct {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::rng();

        let mut input = [0.0; BIG_STRUCT_SIZE];
        let mut out = [0.0; BIG_STRUCT_SIZE];

        for i in 0..size {
            input[i] = rng.random();
            out[i] = rng.random();
        }

        let time = Instant::now();
        BigStruct { input, out, time }
    }
}

pub fn shuffle(input: f64, a: usize) -> f64 {
    let m = BASE.exp2() + 1.;
    (a as f64 * input + TRANSLATION).rem_euclid(m)
}

struct EnricherMove {
    a: usize,
}

impl EnricherMove {
    pub fn enrich(&self, mut big: BigStruct) -> BigStruct {
        for _ in 0..ITERATIONS {
            for i in 0..big.input.len() {
                let temp = big.out[i];
                big.out[i] = shuffle(big.input[i], self.a);
                big.input[i] = temp;
            }
        }
        big.time = Instant::now();
        big
    }
}

pub struct EnrichersMove(Vec<EnricherMove>);

impl EnrichersMove {
    pub fn new(num: usize) -> Self {
        EnrichersMove((0..num).map(|a| EnricherMove { a }).collect())
    }

    pub fn enrich(&self, mut big: BigStruct) -> BigStruct {
        for enricher in &self.0 {
            big = enricher.enrich(big)
        }
        big
    }
}

struct EnricherRcRefCell {
    a: usize,
}

impl EnricherRcRefCell {
    pub fn enrich(&self, big: Rc<RefCell<BigStruct>>) {
        let mut big = big.borrow_mut();
        for _ in 0..ITERATIONS {
            for i in 0..big.input.len() {
                let temp = big.out[i];
                big.out[i] = shuffle(big.input[i], self.a);
                big.input[i] = temp;
            }
        }
        big.time = Instant::now();
    }
}

pub struct EnrichersRcRefCell(Vec<EnricherRcRefCell>);

impl EnrichersRcRefCell {
    pub fn new(num: usize) -> Self {
        EnrichersRcRefCell((0..num).map(|a| EnricherRcRefCell { a }).collect())
    }

    pub fn enrich(&self, big: Rc<RefCell<BigStruct>>) -> Rc<RefCell<BigStruct>> {
        for enricher in &self.0 {
            enricher.enrich(big.clone())
        }
        big
    }
}
