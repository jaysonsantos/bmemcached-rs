#[macro_use]
extern crate criterion;
extern crate bmemcached;

use criterion::Criterion;

use bmemcached::protocol::Protocol;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("set/get/delete", |b| {
        let mut cli = Protocol::connect("127.0.0.1:11211").unwrap();
        b.iter(|| {
            let key = "benchmark test";
            cli.set(key, "abc", 10_000).unwrap();
            let returned_value: String = cli.get(key).unwrap();
            assert_eq!(returned_value, "abc");
            cli.delete(key).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
