// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct X { x: (), }

impl X : Drop {
    fn finalize(&self) {
        error!("destructor runs");
    }
}

fn main() {
    let x = Some(X { x: () });
    match move x {
        Some(move _y @ ref _z) => { }, //~ ERROR cannot bind by-move with sub-bindings
        None => die!()
    }
}
