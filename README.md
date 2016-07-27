# fsort
fsort is a crate to sort files in a fast, OS-independent and 'rusty' way.

## Features
- Easy sorting
- Many criteria: Name, size, creation date, access date, ...
- Pure Rust implementation without depedencies
- OS-independent

## Example
```rust
use std::fs::File;
use std::path::PathBuf;
use fsort::criterion::{FileName, FileSize};
use fsort::file_collection::{FileCollection, DynamicCollection};

fn main() {

  // Create temporal files
  let mut s1 = std::env::temp_dir();
  let mut s2 = std::env::temp_dir();
  s1.push("S1.tmp");
  s2.push("S2.tmp");
  File::create(&s1).unwrap().set_len(10);
  File::create(&s2).unwrap().set_len(5);

  // Inserts files into collection
  let mut collection = DynamicCollection::new::<FileName>();
  collection.add_file(&s2);
  collection.add_file(&s1);

  // Sort files by name and iterate over the paths
  let mut iter_name = collection.path_iter();
  assert_eq!(s1, iter_name.next().unwrap());
  assert_eq!(s2, iter_name.next().unwrap());
  assert_eq!(None, iter_name.next());

  // Change sort criterion and iterate again
  collection.set_criterion::<FileSize>();
  let mut iter_size = collection.path_iter();
  assert_eq!(s2, iter_size.next().unwrap());
  assert_eq!(s1, iter_size.next().unwrap());
  assert_eq!(None, iter_size.next());
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
