# pbuf
Reads/Writes to anything that implements Read/Write to/from protobuf...

Examples assume `Msg` is your protobuf and msg.rs is where it lives, see [rust-protobuf](https://github.com/stepancheg/rust-protobuf) on how to generate `.rs` from `.proto` files.

### Add to Cargo.toml
```
pbuf = "0.3.2"
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
