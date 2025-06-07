use criterion::{criterion_group, criterion_main, Criterion};
use midas_core::{simulate, model};

fn bench_simulation(c: &mut Criterion) {
    let index_data = vec![
        model::IndexData { date: "2022-10-11".to_string(), close_point: 10577.81 },
        model::IndexData { date: "2022-10-12".to_string(), close_point: 10838.48 },
        model::IndexData { date: "2022-10-13".to_string(), close_point: 10817.67 },
        model::IndexData { date: "2022-10-14".to_string(), close_point: 11121.72 },
        model::IndexData { date: "2022-10-17".to_string(), close_point: 11162.26 },
        model::IndexData { date: "2022-10-18".to_string(), close_point: 11187.70 },
        model::IndexData { date: "2022-10-19".to_string(), close_point: 11027.24 },
        model::IndexData { date: "2022-10-20".to_string(), close_point: 10965.33 },
        model::IndexData { date: "2022-10-21".to_string(), close_point: 10918.97 },
        model::IndexData { date: "2022-10-24".to_string(), close_point: 10694.61 },
        model::IndexData { date: "2022-10-25".to_string(), close_point: 10639.82 },
        model::IndexData { date: "2022-10-26".to_string(), close_point: 10818.33 },
        model::IndexData { date: "2022-10-27".to_string(), close_point: 10750.14 },
        model::IndexData { date: "2022-10-28".to_string(), close_point: 10401.84 },
        model::IndexData { date: "2022-10-31".to_string(), close_point: 10397.04 },
        model::IndexData { date: "2022-11-01".to_string(), close_point: 10734.25 },
        model::IndexData { date: "2022-11-02".to_string(), close_point: 10877.51 },
        model::IndexData { date: "2022-11-03".to_string(), close_point: 10840.06 },
        model::IndexData { date: "2022-11-04".to_string(), close_point: 11187.43 },
        model::IndexData { date: "2022-11-07".to_string(), close_point: 11207.73 },
        model::IndexData { date: "2022-11-08".to_string(), close_point: 11142.93 },
        model::IndexData { date: "2022-11-09".to_string(), close_point: 11055.29 },
        model::IndexData { date: "2022-11-10".to_string(), close_point: 10908.55 },
        model::IndexData { date: "2022-11-11".to_string(), close_point: 11117.45 },
        model::IndexData { date: "2022-11-14".to_string(), close_point: 11238.15 },
        model::IndexData { date: "2022-11-15".to_string(), close_point: 11323.35 },
        model::IndexData { date: "2022-11-16".to_string(), close_point: 11247.86 },
        model::IndexData { date: "2022-11-17".to_string(), close_point: 11174.54 },
        model::IndexData { date: "2022-11-18".to_string(), close_point: 11192.81 },
        model::IndexData { date: "2022-11-21".to_string(), close_point: 11019.79 },
        model::IndexData { date: "2022-11-22".to_string(), close_point: 10930.28 },
        model::IndexData { date: "2022-11-23".to_string(), close_point: 10958.55 },
        model::IndexData { date: "2022-11-24".to_string(), close_point: 11026.59 },
        model::IndexData { date: "2022-11-25".to_string(), close_point: 11073.87 },
        model::IndexData { date: "2022-11-28".to_string(), close_point: 10934.13 },
        model::IndexData { date: "2022-11-29".to_string(), close_point: 10881.20 },
        model::IndexData { date: "2022-11-30".to_string(), close_point: 11014.62 },
        model::IndexData { date: "2022-12-01".to_string(), close_point: 11212.19 },
        model::IndexData { date: "2022-12-02".to_string(), close_point: 11340.90 },
        model::IndexData { date: "2022-12-05".to_string(), close_point: 11296.27 },
        model::IndexData { date: "2022-12-06".to_string(), close_point: 11323.33 },
        model::IndexData { date: "2022-12-07".to_string(), close_point: 11323.35 },
        model::IndexData { date: "2022-12-08".to_string(), close_point: 11296.45 },
        model::IndexData { date: "2022-12-09".to_string(), close_point: 11323.47 },
        model::IndexData { date: "2022-12-12".to_string(), close_point: 11323.49 },
        model::IndexData { date: "2022-12-13".to_string(), close_point: 11323.51 },
        model::IndexData { date: "2022-12-14".to_string(), close_point: 11323.53 },
        model::IndexData { date: "2022-12-15".to_string(), close_point: 11323.55 },
        model::IndexData { date: "2022-12-16".to_string(), close_point: 11323.57 },
        model::IndexData { date: "2022-12-19".to_string(), close_point: 11323.59 },
        model::IndexData { date: "2022-12-20".to_string(), close_point: 11323.61 },
        model::IndexData { date: "2022-12-21".to_string(), close_point: 11323.63 },
        model::IndexData { date: "2022-12-22".to_string(), close_point: 11323.65 },
        model::IndexData { date: "2022-12-23".to_string(), close_point: 11323.67 },
        model::IndexData { date: "2022-12-26".to_string(), close_point: 11323.69 },
        model::IndexData { date: "2022-12-27".to_string(), close_point: 11323.71 },
        model::IndexData { date: "2022-12-28".to_string(), close_point: 11323.73 },
        model::IndexData { date: "2022-12-29".to_string(), close_point: 11323.75 },
        model::IndexData { date: "2022-12-30".to_string(), close_point: 11323.77 },
    ];

    c.bench_function("simulation", |b| {
        b.iter(|| {
            simulate::simulate(
                100000.0, 20, 0.93, 1.07, 0.0003, &index_data
            )
        })
    });
}

criterion_group!(benches, bench_simulation);
criterion_main!(benches);