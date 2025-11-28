use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serde::{Deserialize, Serialize};
use wincode::{containers::Pod, deserialize, serialize, SchemaRead, SchemaWrite};

#[path = "utils.rs"]
mod utils;

#[repr(transparent)]
#[derive(Clone, Copy, SchemaWrite, SchemaRead, Serialize, Deserialize)]
struct PodStruct([u8; 32]);

fn bench_vec_pod_vs_regular_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec serialize: Pod<u64> vs u64");
    
    // Note: wincode already optimizes Vec<u64> serialization by treating u64 as a primitive
    // that can be written directly. Therefore, we expect regular Vec<u64> and Vec<Pod<u64>>
    // to have very similar performance characteristics. The Pod wrapper is most beneficial
    // for custom structs that are safe for zero-copy but not automatically detected as such.
    
    for size in [100, 1_000, 10_000, 100_000] {
        let data = utils::generate_vec_u64(size);
        let bytes = (size * 8) as u64;
        
        group.throughput(Throughput::Bytes(bytes));
        group.bench_with_input(
            BenchmarkId::new("regular", size),
            &data,
            |b, data| {
                b.iter(|| serialize(black_box(data)).unwrap());
            },
        );
        
        #[derive(SchemaWrite, SchemaRead)]
        struct WithPod {
            #[wincode(with = "wincode::containers::Vec<Pod<_>>")]
            data: Vec<u64>,
        }
        
        let with_pod = WithPod { data: data.clone() };
        group.bench_with_input(
            BenchmarkId::new("Pod optimized", size),
            &with_pod,
            |b, data| {
                b.iter(|| serialize(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_vec_pod_vs_regular_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec deserialize: Pod<u64> vs u64");
    
    for size in [100, 1_000, 10_000, 100_000] {
        let data = utils::generate_vec_u64(size);
        let bytes = (size * 8) as u64;
        
        let serialized_regular = serialize(&data).unwrap();
        group.throughput(Throughput::Bytes(bytes));
        group.bench_with_input(
            BenchmarkId::new("regular", size),
            &serialized_regular,
            |b, serialized| {
                b.iter(|| deserialize::<Vec<u64>>(black_box(serialized)).unwrap());
            },
        );
        
        #[derive(SchemaWrite, SchemaRead)]
        struct WithPod {
            #[wincode(with = "wincode::containers::Vec<Pod<_>>")]
            data: Vec<u64>,
        }
        
        let with_pod = WithPod { data: data.clone() };
        let serialized_pod = serialize(&with_pod).unwrap();
        group.bench_with_input(
            BenchmarkId::new("Pod optimized", size),
            &serialized_pod,
            |b, serialized| {
                b.iter(|| deserialize::<WithPod>(black_box(serialized)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_vec_pod_struct_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec<PodStruct> serialize");
    
    for size in [100, 1_000, 10_000] {
        let data: Vec<PodStruct> = (0..size)
            .map(|i| PodStruct([i as u8; 32]))
            .collect();
        let bytes = (size * 32) as u64;
        
        group.throughput(Throughput::Bytes(bytes));
        group.bench_with_input(
            BenchmarkId::new("regular", size),
            &data,
            |b, data| {
                b.iter(|| serialize(black_box(data)).unwrap());
            },
        );
        
        #[derive(SchemaWrite, SchemaRead)]
        struct WithPodStruct {
            #[wincode(with = "wincode::containers::Vec<Pod<_>>")]
            data: Vec<PodStruct>,
        }
        
        let with_pod = WithPodStruct { data: data.clone() };
        group.bench_with_input(
            BenchmarkId::new("Pod optimized", size),
            &with_pod,
            |b, data| {
                b.iter(|| serialize(black_box(data)).unwrap());
            },
        );
    }
    group.finish();
}

fn bench_vec_pod_struct_deserialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec<PodStruct> deserialize");
    
    for size in [100, 1_000, 10_000] {
        let data: Vec<PodStruct> = (0..size)
            .map(|i| PodStruct([i as u8; 32]))
            .collect();
        let bytes = (size * 32) as u64;
        
        let serialized_regular = serialize(&data).unwrap();
        group.throughput(Throughput::Bytes(bytes));
        group.bench_with_input(
            BenchmarkId::new("regular", size),
            &serialized_regular,
            |b, serialized| {
                b.iter(|| deserialize::<Vec<PodStruct>>(black_box(serialized)).unwrap());
            },
        );
        
        #[derive(SchemaWrite, SchemaRead)]
        struct WithPodStruct {
            #[wincode(with = "wincode::containers::Vec<Pod<_>>")]
            data: Vec<PodStruct>,
        }
        
        let with_pod = WithPodStruct { data: data.clone() };
        let serialized_pod = serialize(&with_pod).unwrap();
        group.bench_with_input(
            BenchmarkId::new("Pod optimized", size),
            &serialized_pod,
            |b, serialized| {
                b.iter(|| deserialize::<WithPodStruct>(black_box(serialized)).unwrap());
            },
        );
    }
    group.finish();
}

criterion_group!(
    pod_optimization,
    bench_vec_pod_vs_regular_serialize,
    bench_vec_pod_vs_regular_deserialize,
    bench_vec_pod_struct_serialize,
    bench_vec_pod_struct_deserialize,
);

criterion_main!(pod_optimization);
