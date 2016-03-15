# pbuf
Rust pbuf, std/file reader/writer

Examples assume `Msg` is your protobuf and msg.rs is where it lives, see [rust-protobuf](https://github.com/stepancheg/rust-protobuf) on how to generate `.rs` from `.proto` files.

### Reading example

```
extern crate pbuf;
extern crate protobuf;

use std::io::prelude::*;
use std::io::{BufReader, self};
use std::fs::File;

mod msg
use msg::Msg

let mut reader = Box::new(BufReader::new(io::stdin()));
// To read from file 
// let mut reader = Box::new(BufReader::new(File::open  (&<file pointer>).unwrap()))

// Reads entire buffer and fill
let mut msgs: Vec<Msg> = read_pbuf(&mut reader);
```

### Writing example
```
extern crate pbuf;
extern crate protobuf;

use std::io::prelude::*;
use std::io::{BufWriter, self};
use std::fs::File;

mod msg
use msg::Msg

let mut writer = Box::new(BufWriter::new(io::stdout()));
// To write to file 
// let mut writer = Box::new(BufWriter::new(File::create(&<file pointer>).unwrap()))

for msg in msgs.iter() {
    write_pbuf(msg, &mut writer);
  }
```
