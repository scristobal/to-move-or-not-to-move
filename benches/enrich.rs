use std::{cell::RefCell, rc::Rc};

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use move_or_not::{BIG_STRUCT_SIZE, BigStruct, EnrichersMove, EnrichersRcRefCell, NUM_ENRICHERS};

pub fn criterion_benchmark(c: &mut Criterion) {
    let enrichers = EnrichersMove::new(NUM_ENRICHERS);

    c.bench_function("enrich by move BigStruct", |b| {
        b.iter(|| {
            let big = BigStruct::new(black_box(BIG_STRUCT_SIZE));
            enrichers.enrich(big)
        })
    });

    let enrichers = EnrichersRcRefCell::new(NUM_ENRICHERS);

    c.bench_function("enrich by Rc<RefCell<BigStruct>>", |b| {
        b.iter(|| {
            let big = Rc::new(RefCell::new(BigStruct::new(black_box(BIG_STRUCT_SIZE))));
            enrichers.enrich(big)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
