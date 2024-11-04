use criterion::{black_box, criterion_group, criterion_main, Criterion};
use profinet_rt::ProfinetPacket;
use std::convert::TryFrom;

fn benchmark_profinet_packet_conversion(c: &mut Criterion) {
    let data: Vec<u8> = vec![
        0xFE, 0xFE, // Frame ID
        0x01, 0x02, // Service ID and Service Type
        0x00, 0x00, 0x00, 0x01, // XID
        0x00, 0x10, // Response Delay
        0x00, 0x0C, // DCP Data Length
        0x02, // Option
        0x03, // Suboption
        0x00, 0x04, // DCP Block Length
        b'T', b'E', b'S', b'T' // Name Of Station
    ];

    c.bench_function("profinet_packet_conversion", |b| {
        b.iter(|| {
            let _ = ProfinetPacket::try_from(black_box(&data[..])).expect("Failed to parse packet");
        });
    });
}

criterion_group!(benches, benchmark_profinet_packet_conversion);
criterion_main!(benches);
