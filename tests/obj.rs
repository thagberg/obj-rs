// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate obj;

use std::fs::File;
use std::io::BufReader;
use obj::*;

macro_rules! fixture {
    ($name:expr) => (BufReader::new(File::open(concat!("tests/fixtures/", $name)).unwrap()))
}

#[test]
fn normal_cone() {
    let obj: Obj = load_obj(fixture!("normal-cone.obj")).unwrap();

    macro_rules! v {
        (($($p:expr),*), ($($n:expr),*)) => ({
            Vertex {
                position: [$(stringify!($p).parse::<f32>().unwrap()),*],
                normal: [$(stringify!($n).parse::<f32>().unwrap()),*],
                uv: [0.0; 3],
            }
        })
    }

    assert_eq!(obj.name, Some("Cone".to_string()));
    assert_eq!(obj.vertices.len(), 96);
    assert_eq!(obj.vertices[0], v!((-0.382682, -1.000000, -0.923880), (-0.259887, 0.445488, -0.856737)));
    assert_eq!(obj.vertices[1], v!(( 0.000000,  1.000000,  0.000000), (-0.259887, 0.445488, -0.856737)));
    assert_eq!(obj.indices.len(), 96);
    assert_eq!(obj.indices[0], 0);
    assert_eq!(obj.indices[1], 1);
    assert_eq!(obj.indices[2], 2);
}

#[test]
fn dome() {
    let obj: Obj<Position> = load_obj(fixture!("dome.obj")).unwrap();

    macro_rules! p {
        ($($x:expr),*) => (Position { position: [$(stringify!($x).parse::<f32>().unwrap()),*] })
    }

    assert_eq!(obj.name, Some("Dome".to_string()));
    assert_eq!(obj.vertices[0], p!(-0.382683, 0.923880, 0.000000));
    assert_eq!(obj.vertices[1], p!(-0.707107, 0.707107, 0.000000));
    assert_eq!(obj.indices[0], 3);
    assert_eq!(obj.indices[1], 2);
    assert_eq!(obj.indices[2], 6);
}
