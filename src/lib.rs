/*!A simple configuration parsing utility with no dependencies built on Rust.
`configparser` works on a subset of ini configuration syntax. It is inspired by Python's `configparser`.
The current release is experimental, this means that future releases will be swift until we reach `stable` (1.0.0).
The codebase is thus subject to change for now.

## ini-style configuration

Most `ini` files look something like this (but they don't need to be `.ini` files obviously):
```yaml
[some-section]
key1 = value1
key2 = value2

[some-other-section]
key3 = value3
maybekey1aswell = value1
```
Owing to how ini files usually are, this means that `[`, `]` and `=` are special symbols (this crate will allow you to use `]` sparingly). Duplicate
values are not allowed, only the last key is stored and this applies to section headers and section keys alike.

Key-value pairs or section headers cannot spread across multiple lines for obvious reasons because the parser cannot reliably parse it otherwise.
A value on the next line could as well be a key for another.

An important note is that key-value pairs not attached to any section are automatically put in a section called `DEFAULT`.
Future releases will add support for escaping, comments and modifying default section naming.

## Syntax

You can load a `HashMap` easily and parse it like:
```ignore,rust
use configparser::ini::Ini;
use std::collections::HashMap;

fn main() {
  let mut config = Ini::new();

  match config.load("Path/to/file...") {
  	Err(why) => panic!("{}", why),
  	Ok(_) => println!("Yay!")
  }

  // You can then access the map normally like:
  let map = match config.get_map() {
  	None => HashMap::new(), // or whatever you want to do if the map is empty
  	Some(map) => map
  }; // or let map = config.get_map()? instead of match

  for (key, value) in &map {
      println!("{:?}: {:?}", key, value);
  }
  // And safely fetch a value:
  let val = config.get("section name", "key name")?; // or use .unwrap()
}
```
The `Ini` struct is the way to go forward and will soon have more features, such as reading from a string, insertion, deletion and variable access.
*/
pub mod ini;