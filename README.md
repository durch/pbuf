[![](https://camo.githubusercontent.com/2fee3780a8605b6fc92a43dab8c7b759a274a6cf/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f72757374632d737461626c652d627269676874677265656e2e737667)](https://www.rust-lang.org/downloads.html)
[![Build Status](https://travis-ci.org/durch/pbuf.svg?branch=master)](https://travis-ci.org/durch/pbuf)
[![](http://meritbadge.herokuapp.com/pbuf)](https://crates.io/crates/pbuf)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/durch/pbuf/blob/master/LICENSE.md)
[![Join the chat at https://gitter.im/durch/pbuf](https://badges.gitter.im/durch/rust-s3.svg)](https://gitter.im/durch/pbuf?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)


# pbuf [[docs](https://durch.github.io/pbuf/)]
Reads/Writes to anything that implements Read/Write to/from protobuf...

Examples assume `Msg` is your protobuf and msg.rs is where it lives, see [rust-protobuf](https://github.com/stepancheg/rust-protobuf) on how to generate `.rs` from `.proto` files.

### Add to Cargo.toml
```
pbuf = "^0.3"
```

### Read example

```
extern crate pbuf;
extern crate protobuf;

use std::io::prelude::*;
use std::io::{BufReader, self};
use std::fs::File;

mod msg;
use msg::Msg;

use pbuf::read_pbuf;

let mut reader = Box::new(BufReader::new(io::stdin()));
// To read from file
// let mut reader = Box::new(BufReader::new(File::open  (&<file pointer>).unwrap()))

// Reads entire buffer and fill
let mut msgs: Vec<Msg> = read_pbuf(&mut reader);
```

### Write example
```
extern crate pbuf;
extern crate protobuf;

use std::io::prelude::*;
use std::io::{BufWriter, self};
use std::fs::File;

mod msg;
use msg::Msg;

use pbuf::write_pbuf;

let mut writer = Box::new(BufWriter::new(io::stdout()));
// To write to file
// let mut writer = Box::new(BufWriter::new(File::create(&<file pointer>).unwrap()))

for msg in msgs.iter() {
    write_pbuf(msg, &mut writer);
  }
```
