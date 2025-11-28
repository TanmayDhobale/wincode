use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

#[path = "utils.rs"]
mod utils;

use utils::{SimpleStruct, ComplexStruct};

fn bench_simple_struct_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("SimpleStruct::serialize");
    group.throughput(Throughput::Elements(1));
    let data = SimpleStruct::default();
    
    let wincode_bytes = wincode::serialize(&data).unwrap();
    let bincode_bytes = bincode::serialize(&data).unwrap();
    assert_eq!(wincode_bytes, bincode_bytes, "Serialized data must match!");
    
    group.bench_function("wincode", |b| {
        b.iter(|| wincode::serialize(black_box(&data)).unwrap());
    });
    
    group.bench_function("bincode", |b| {
        b.iter(|| bincode::serialize(black_box(&data)).unwrap());
    });
    
    group.finish();
}

fn bench_simple_struct_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("SimpleStruct::deserialize");
    group.throughput(Throughput::Elements(1));
    let data = SimpleStruct::default();
    let serialized = bincode::serialize(&data).unwrap();
    assert_eq!(wincode::serialize(&data).unwrap(), serialized, "Serialized data must match!");
    
    group.bench_function("wincode", |b| {
        b.iter(|| wincode::deserialize::<SimpleStruct>(black_box(&serialized)).unwrap());
    });
    
    group.bench_function("bincode", |b| {
        b.iter(|| bincode::deserialize::<SimpleStruct>(black_box(&serialized)).unwrap());
    });
    
    group.finish();
}

fn bench_vec_u64_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec<u64>::serialize comparison");
    
    for size in [100, 1_000, 10_000] {
        let data = utils::generate_vec_u64(size);
        group.throughput(Throughput::Bytes((size * 8) as u64));
        
        let wincode_bytes = wincode::serialize(&data).unwrap();
        let bincode_bytes = bincode::serialize(&data).unwrap();
        assert_eq!(wincode_bytes, bincode_bytes, "Serialized Vec<u64> must match!");
        
        group.bench_with_input(
            BenchmarkId::new("wincode", size),
            &data,
            |b, data| {
                b.iter(|| wincode::serialize(black_box(data)).unwrap());
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("bincode", size),
            &data,
            |b, data| {
                b.iter(|| bincode::serialize(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_vec_u64_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec<u64>::deserialize comparison");
    
    for size in [100, 1_000, 10_000] {
        let data = utils::generate_vec_u64(size);
        group.throughput(Throughput::Bytes((size * 8) as u64));
        
        let serialized = bincode::serialize(&data).unwrap();
        assert_eq!(wincode::serialize(&data).unwrap(), serialized, "Serialized Vec<u64> must match!");
        
        group.bench_with_input(
            BenchmarkId::new("wincode", size),
            &serialized,
            |b, data| {
                b.iter(|| wincode::deserialize::<Vec<u64>>(black_box(data)).unwrap());
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("bincode", size),
            &serialized,
            |b, data| {
                b.iter(|| bincode::deserialize::<Vec<u64>>(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_complex_struct_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("ComplexStruct::serialize");
    
    for size in [100, 1_000, 10_000] {
        let data = ComplexStruct::new(size);
        group.throughput(Throughput::Bytes(size as u64));
        
        let wincode_bytes = wincode::serialize(&data).unwrap();
        let bincode_bytes = bincode::serialize(&data).unwrap();
        assert_eq!(wincode_bytes, bincode_bytes, "Serialized ComplexStruct must match!");
        
        group.bench_with_input(
            BenchmarkId::new("wincode", size),
            &data,
            |b, data| {
                b.iter(|| wincode::serialize(black_box(data)).unwrap());
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("bincode", size),
            &data,
            |b, data| {
                b.iter(|| bincode::serialize(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_complex_struct_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("ComplexStruct::deserialize");
    
    for size in [100, 1_000, 10_000] {
        let data = ComplexStruct::new(size);
        group.throughput(Throughput::Bytes(size as u64));
        
        let serialized = bincode::serialize(&data).unwrap();
        assert_eq!(wincode::serialize(&data).unwrap(), serialized, "Serialized ComplexStruct must match!");
        
        group.bench_with_input(
            BenchmarkId::new("wincode", size),
            &serialized,
            |b, data| {
                b.iter(|| wincode::deserialize::<ComplexStruct>(black_box(data)).unwrap());
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("bincode", size),
            &serialized,
            |b, data| {
                b.iter(|| bincode::deserialize::<ComplexStruct>(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_string_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("String::serialize comparison");
    
    for size in [100, 1_000, 10_000] {
        let data = utils::generate_string(size);
        group.throughput(Throughput::Bytes(size as u64));
        
        let wincode_bytes = wincode::serialize(&data).unwrap();
        let bincode_bytes = bincode::serialize(&data).unwrap();
        assert_eq!(wincode_bytes, bincode_bytes, "Serialized String must match!");
        
        group.bench_with_input(
            BenchmarkId::new("wincode", size),
            &data,
            |b, data| {
                b.iter(|| wincode::serialize(black_box(data)).unwrap());
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("bincode", size),
            &data,
            |b, data| {
                b.iter(|| bincode::serialize(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_string_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("String::deserialize comparison");
    
    for size in [100, 1_000, 10_000] {
        let data = utils::generate_string(size);
        group.throughput(Throughput::Bytes(size as u64));
        
        let serialized = bincode::serialize(&data).unwrap();
        assert_eq!(wincode::serialize(&data).unwrap(), serialized, "Serialized String must match!");
        
        group.bench_with_input(
            BenchmarkId::new("wincode", size),
            &serialized,
            |b, data| {
                b.iter(|| wincode::deserialize::<String>(black_box(data)).unwrap());
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("bincode", size),
            &serialized,
            |b, data| {
                b.iter(|| bincode::deserialize::<String>(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

criterion_group!(
    comparison_bincode,
    bench_simple_struct_serialize,
    bench_simple_struct_deserialize,
    bench_vec_u64_serialize,
    bench_vec_u64_deserialize,
    bench_complex_struct_serialize,
    bench_complex_struct_deserialize,
    bench_string_serialize,
    bench_string_deserialize,
);

criterion_main!(comparison_bincode);
