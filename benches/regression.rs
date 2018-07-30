#[macro_use]
extern crate criterion;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate bmemcached;

use criterion::Criterion;

use bmemcached::errors::Result;
use bmemcached::protocol::Protocol;
use bmemcached::{FromMemcached, StoredType, ToMemcached};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Data {
    value: usize,
}

impl ToMemcached for Data {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType)> {
        Ok((
            serde_json::to_vec(self).unwrap(),
            StoredType::MTYPE_USER_DEFINED_1,
        ))
    }
}

impl FromMemcached for Data {
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self> {
        assert!(flags == StoredType::MTYPE_USER_DEFINED_1);
        Ok(serde_json::from_slice(&*buf).unwrap())
    }
}

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

    c.bench_function("set/get/delete struct", |b| {
        let mut cli = Protocol::connect("127.0.0.1:11211").unwrap();
        b.iter(|| {
            let key = "benchmark test";
            let data = Data::default();
            cli.set(key, data, 10_000).unwrap();
            let returned_value: Data = cli.get(key).unwrap();
            assert_eq!(returned_value.value, 0);
            cli.delete(key).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
