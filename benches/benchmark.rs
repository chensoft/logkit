#[macro_use] extern crate logkit;

use criterion::*;

fn empty_log(c: &mut Criterion) {
    let logger = logkit::Logger::new();
    logkit::set_default_logger(logger);

    c.bench_function("empty_log", |b| b.iter(|| {
        trace!();
    }));
}

fn level_off(c: &mut Criterion) {
    let mut logger = logkit::Logger::new();
    logger.level = logkit::LEVEL_OFF;
    logkit::set_default_logger(logger);

    c.bench_function("level_off", |b| b.iter(|| {
        trace!();
    }));
}

fn msg_only(c: &mut Criterion) {
    let logger = logkit::Logger::new();
    logkit::set_default_logger(logger);

    c.bench_function("msg_only", |b| b.iter(|| {
        trace!("Hi Alice! It's been 2 years since our last trip together.");
    }));
}

fn msg_format(c: &mut Criterion) {
    let logger = logkit::Logger::new();
    logkit::set_default_logger(logger);

    c.bench_function("msg_format", |b| b.iter(|| {
        trace!("Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    }));
}

fn fields_only(c: &mut Criterion) {
    let logger = logkit::Logger::new();
    logkit::set_default_logger(logger);

    c.bench_function("fields_only", |b| b.iter(|| {
        trace!(name = "Alice", age = 20, time = 1706098776, pi = std::f32::consts::PI);
    }));
}

fn fields_msg(c: &mut Criterion) {
    let logger = logkit::Logger::new();
    logkit::set_default_logger(logger);

    c.bench_function("fields_msg", |b| b.iter(|| {
        trace!(name = "Alice", age = 20, time = 1706098776, pi = std::f32::consts::PI; "Hi Alice! It's been 2 years since our last trip together.");
    }));
}

fn fields_msg_format(c: &mut Criterion) {
    let logger = logkit::Logger::new();
    logkit::set_default_logger(logger);

    c.bench_function("fields_msg_format", |b| b.iter(|| {
        trace!(name = "Alice", age = 20, time = 1706098776, pi = std::f32::consts::PI; "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    }));
}

criterion_group!(
    benches,
    empty_log,
    level_off,
    msg_only,
    msg_format,
    fields_only,
    fields_msg,
    fields_msg_format,
);
criterion_main!(benches);