// TODO unclear why we need these extern crates here?
extern crate criterion;
extern crate pprof;
extern crate roslibrust_serde_rosmsg;
extern crate serde;

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

const IMAGE_DATA: &[u8] = include_bytes!("../src/datatests/sensor_msgs_image_1080p.bin");

use pprof::criterion::{Output, PProfProfiler};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Header {
    pub seq: u32,
    pub stamp: Time,
    pub frame_id: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Time {
    pub secs: u32,
    pub nsecs: u32,
}

// Basic Image Representation
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VecImage {
    pub header: Header,
    pub height: u32,
    pub width: u32,
    pub encoding: String,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: Vec<u8>,
}

// Includes serde_bytes optimization
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct VecBytesImage {
    pub header: Header,
    pub height: u32,
    pub width: u32,
    pub encoding: String,
    pub is_bigendian: u8,
    pub step: u32,
    // serde_bytes optimization here makes deserialization of an image ~97% faster
    // Without it deserializing a 1080p color image took ~22.2ms on a Ryzen 3950x
    // With it that drops to 520us on the same system
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

// Below two options not currently supported
// // No serde_bytes optimization referenced data instead of copying it
// // Note: Deserializer is not really setup to take advantage of this yet
// #[derive(Deserialize, Serialize, PartialEq, Debug)]
// pub struct RefImage<'a> {
//     pub header: Header,
//     pub height: u32,
//     pub width: u32,
//     pub encoding: String,
//     pub is_bigendian: u8,
//     pub step: u32,
//     pub data: &'a [u8],
// }

// // With serde_bytes optimization, on referenced data
// #[derive(Deserialize, Serialize, PartialEq, Debug)]
// pub struct RefBytesImage<'a> {
//     pub header: Header,
//     pub height: u32,
//     pub width: u32,
//     pub encoding: String,
//     pub is_bigendian: u8,
//     pub step: u32,
//     #[serde(with = "serde_bytes")]
//     pub data: &'a [u8],
// }

// An alternate expression option that also works
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct SharedImage {
    pub header: Header,
    pub height: u32,
    pub width: u32,
    pub encoding: String,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: Box<[u8]>,
}

#[inline]
fn parse_vec_image() {
    let image: VecImage = roslibrust_serde_rosmsg::from_slice(IMAGE_DATA).unwrap();
    black_box(image);
}

#[inline]
fn parse_vec_bytes_image() {
    let image: VecBytesImage = roslibrust_serde_rosmsg::from_slice(IMAGE_DATA).unwrap();
    black_box(image);
}

#[inline]
fn parse_shared_image() {
    let image: SharedImage = roslibrust_serde_rosmsg::from_slice(IMAGE_DATA).unwrap();
    black_box(image);
}

#[inline]
fn serialize_vec_bytes_image(image: &VecBytesImage) {
    black_box(roslibrust_serde_rosmsg::to_vec(image).unwrap());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_vec_image", |b| b.iter(|| parse_vec_image()));
    c.bench_function("parse_vec_bytes_image", |b| {
        b.iter(|| parse_vec_bytes_image())
    });
    c.bench_function("parse_shared_image", |b| b.iter(|| parse_shared_image()));

    let image: VecBytesImage = roslibrust_serde_rosmsg::from_slice(IMAGE_DATA).unwrap();
    c.bench_function("serialize_vec_bytes_image", |b| {
        b.iter(|| serialize_vec_bytes_image(&image))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
    targets = criterion_benchmark
);
criterion_main!(benches);
