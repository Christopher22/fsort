# fsort [![Build Status](https://travis-ci.org/Christopher22/fsort.svg?branch=master)](https://travis-ci.org/Christopher22/fsort)
fsort is a crate to sort files in a fast, OS-independent and 'rusty' way.

## Documentation
[Documentation on GitHub](https://christopher22.github.io/fsort/fsort/)

## Features
- Easy sorting
- Many criteria: Name, size, creation date, access date, ...
- Pure Rust implementation without depedencies
- OS-independent

## Example
```rust
use std::path::{Path, PathBuf};
use fsort::criterion::{FileName, FileSize};
use fsort::file_collection::{FileCollection, DynamicCollection};

fn main() {

  // Create a dynamic collection
  let mut collection = DynamicCollection::new::<FileName>();
  
  // Add paths
  collection.add_file(&Path::new("a_file.tmp").to_owned());
  collection.add_file(&Path::new("another_file.tmp").to_owned());
  
  // Files are now sorted by name... Well, change crition to size.
  collection.set_criterion::<FileSize>();
  
  // Iterate sorted files
  for path in collection {
     // Do fancy things with the PathBuf...
  }
}
```
##Author
Christopher Gundler (<c.gundler@mail.de>)

##License
Licensed under either of
 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)
at your option.

##Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
