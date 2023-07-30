/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
include!("bindings.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn can_calculate_gc() {
        let egll = crate::geo_pos2_t {
            lat: 51.468,
            lon: -0.4551,
        };

        let kjfk = crate::geo_pos2_t {
            lat: 40.6398,
            lon: -73.7789,
        };
        unsafe {
            let distance = crate::gc_distance(egll, kjfk);
            assert_eq!((distance / 1852.0).round(), 3000.0);
        }
    }
}
