use std::{cell::RefCell, rc::Rc, time::Duration};

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use move_or_not::{
    BIG_STRUCT_SIZE, BigStruct, EnrichersMove, EnrichersMutRef, EnrichersRcRefCell, NUM_ENRICHERS,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("To move or not to move");
    group.measurement_time(Duration::from_secs(25));
    let enrichers = EnrichersMove::new(NUM_ENRICHERS);

    group.bench_function("enrich by move BigStruct", |b| {
        b.iter(|| {
            let big = BigStruct::new(black_box(BIG_STRUCT_SIZE));
            enrichers.enrich(big)
        })
    });

    let enrichers = EnrichersRcRefCell::new(NUM_ENRICHERS);

    group.bench_function("enrich by Rc<RefCell<BigStruct>>", |b| {
        b.iter(|| {
            let big = Rc::new(RefCell::new(BigStruct::new(black_box(BIG_STRUCT_SIZE))));
            enrichers.enrich(big)
        })
    });

    let enrichers = EnrichersMutRef::new(NUM_ENRICHERS);

    group.bench_function("enrich by mutable reference of BigStruct", |b| {
        b.iter(|| {
            let mut big = BigStruct::new(black_box(BIG_STRUCT_SIZE));
            enrichers.enrich(&mut big)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
