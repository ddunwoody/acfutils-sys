/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

use acfutils_sys::{gc_distance, geo_pos2_t};

fn main() {
    let egll = geo_pos2_t {
        lat: 51.468,
        lon: -0.4551,
    };

    let kjfk = geo_pos2_t {
        lat: 40.6398,
        lon: -73.7789,
    };

    unsafe {
        let distance = gc_distance(egll, kjfk);
        println!(
            "The distance from London to New York is {:.1}nm",
            distance / 1852.0
        );
    }
}
