use criterion::{criterion_group, criterion_main, Criterion, Throughput, black_box};

fn vector(c: &mut Criterion) {
    use tls_codec::*;

    let long_vector = TlsVecU32::from(vec![77u8; 65535]);
    let mut group = c.benchmark_group("TLS Vectors");
    group.throughput(Throughput::Bytes(long_vector.len() as u64));

    group.bench_with_input(
        "Serialize",
        &long_vector,
        |b, long_vec| b.iter(|| black_box(long_vec.tls_serialize_detached().unwrap())),
    );

    let ser_long_vec = TlsSliceU32(long_vector.as_slice()).tls_serialize_detached().unwrap();

    group.bench_with_input("Deserialize", &ser_long_vec, |b, ser_long_vec| {
        b.iter(|| black_box(TlsVecU32::<u8>::tls_deserialize(&mut ser_long_vec.as_slice()).unwrap()))
    });

    // group.bench_with_input("Deserialize", &long_vector, |b, long_vec| {
    //     b.iter_batched(
    //         || TlsSliceU32(long_vec.as_slice()).tls_serialize_detached().unwrap(),
    //         |ser_long_vec| black_box(TlsVecU32::<u8>::tls_deserialize(&mut ser_long_vec.as_slice()).unwrap()),
    //         BatchSize::SmallInput,
    //     )
    // });

    group.finish()
}

fn byte_vector(c: &mut Criterion) {
    use tls_codec::*;
    let long_vector = TlsByteVecU32::from(vec![77u8; 65535]);
    let mut group = c.benchmark_group("TLS Byte Vectors");
    group.throughput(Throughput::Bytes(long_vector.len() as u64));

    group.bench_with_input(
        "Serialize",
        &long_vector,
        |b, long_vec| b.iter(|| black_box(long_vec.tls_serialize_detached().unwrap())),
    );

    let ser_long_vec = TlsByteSliceU32(long_vector.as_slice()).tls_serialize_detached().unwrap();

    group.bench_with_input("Deserialize", &ser_long_vec, |b, ser_long_vec| {
        b.iter(|| black_box(TlsVecU32::<u8>::tls_deserialize(&mut ser_long_vec.as_slice()).unwrap()))
    });

    group.finish();
}

fn byte_slice(c: &mut Criterion) {
    use tls_codec::*;
    let long_vector = vec![77u8; 65535];

    let mut group = c.benchmark_group("TLS Byte Slice");
    group.throughput(Throughput::Bytes(long_vector.len() as u64));

    group.bench_with_input(
        "Serialize",
        &long_vector,
        |b, long_vec| b.iter(|| black_box(TlsByteSliceU32(&long_vec).tls_serialize_detached().unwrap())),
    );

    group.finish();
}

fn slice(c: &mut Criterion) {
    use tls_codec::*;

    let long_vector = vec![77u8; 65535];
    let mut group = c.benchmark_group("TLS Slice");
    group.throughput(Throughput::Bytes(long_vector.len() as u64));

    group.bench_with_input(
        "Serialize",
        &long_vector,
        |b, long_vec| b.iter(|| black_box(TlsSliceU32(&long_vec).tls_serialize_detached().unwrap())),
    );

    group.finish();
}
fn benchmark(c: &mut Criterion) {
    vector(c);
    byte_vector(c);
    slice(c);
    byte_slice(c);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
