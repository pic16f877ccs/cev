#![feature(test)]
extern crate test;
use cev::cev::Cev;
use test::{black_box, Bencher};

const LEN: usize = 16384;

#[bench]
fn bench_new(b: &mut Bencher) {
    b.iter(|| Cev::<u32>::new())
}

fn do_bench_with_capacity(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| Cev::<u32>::with_capacity(src_len))
}

#[bench]
fn bench_with_capacity_0000(b: &mut Bencher) {
    do_bench_with_capacity(b, 0)
}

#[bench]
fn bench_with_capacity_0010(b: &mut Bencher) {
    do_bench_with_capacity(b, 10)
}

#[bench]
fn bench_with_capacity_0100(b: &mut Bencher) {
    do_bench_with_capacity(b, 100)
}

#[bench]
fn bench_with_capacity_1000(b: &mut Bencher) {
    do_bench_with_capacity(b, 1000)
}

fn bench_push(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;
    let mut cev = Cev::new();

    b.iter(|| {
        for i in 0..src_len {
            cev.push(i);
        }
    })
}

#[bench]
fn bench_push_0000(b: &mut Bencher) {
    bench_push(b, 0)
}

#[bench]
fn bench_push_0010(b: &mut Bencher) {
    bench_push(b, 10)
}

#[bench]
fn bench_push_0100(b: &mut Bencher) {
    bench_push(b, 100)
}

#[bench]
fn bench_push_1000(b: &mut Bencher) {
    bench_push(b, 1000)
}

fn do_bench_from_fn(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| (0..src_len).collect::<Cev<_>>())
}

#[bench]
fn bench_from_fn_0000(b: &mut Bencher) {
    do_bench_from_fn(b, 0)
}

#[bench]
fn bench_from_fn_0010(b: &mut Bencher) {
    do_bench_from_fn(b, 10)
}

#[bench]
fn bench_from_fn_0100(b: &mut Bencher) {
    do_bench_from_fn(b, 100)
}

#[bench]
fn bench_from_fn_1000(b: &mut Bencher) {
    do_bench_from_fn(b, 1000)
}

#[bench]
fn bench_flat_map_collect(b: &mut Bencher) {
    let v = Cev::from([777u32; 500000]);
    b.iter(|| {
        v.iter()
            .flat_map(|color| color.rotate_left(8).to_be_bytes())
            .collect::<Cev<_>>()
    });
}

#[bench]
fn bench_map_fast(b: &mut Bencher) {
    let data = black_box([(0, 0); LEN]);
    b.iter(|| {
        let mut result: Cev<u32> = Cev::with_capacity(data.len());
        for i in 0..data.len() {
            unsafe {
                *result.as_mut_ptr().sub(i) = data[i].0;
                result.set_len(i);
            }
        }
        result
    });
}

#[bench]
fn bench_range_map_collect(b: &mut Bencher) {
    b.iter(|| (0..LEN).map(|_| u32::default()).collect::<Cev<_>>());
}

#[bench]
fn bench_nest_chain_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        data.iter()
            .cloned()
            .chain([1].iter().chain([2].iter()).cloned())
            .collect::<Cev<_>>()
    });
}

#[bench]
fn bench_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| data.iter().cloned().chain([1]).collect::<Cev<_>>());
}

#[bench]
fn bench_chain_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        data.iter()
            .cloned()
            .chain([1])
            .chain([2])
            .collect::<Cev<_>>()
    });
}

#[derive(Clone)]
struct Droppable(usize);

impl Drop for Droppable {
    fn drop(&mut self) {
        black_box(self);
    }
}

#[bench]
fn bench_in_place_collect_droppable(b: &mut Bencher) {
    let v: Cev<Droppable> = std::iter::repeat_with(|| Droppable(0)).take(1000).collect();
    b.iter(|| {
        v.clone()
            .into_iter()
            .skip(100)
            .enumerate()
            .map(|(i, e)| Droppable(i ^ e.0))
            .collect::<Cev<_>>()
    })
}
