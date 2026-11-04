## EQU (equ)

Equ is simple data package for Rust by w1n4a.
It has trait Core.

## Core:

## `new` - creates new file struct:

```rust
    use equ::Core;

    fn main() {
        let files = core::new("safe.dat"); //new struct
    }
```

## `load` - loads file data as HashMap:

```rust
    use equ::Core;

    fn main() {
        let files = core::new("safe.dat"); //new struct
        let res = files.load().unwrap(); //HashMap
    }
```

## `write` - writes content (HashMap) to file:

```rust
    use equ::Core;
    use std::collections::HashMap;

    fn main() {
        let files = core::new("safe.dat"); //new struct

        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert(String::from("Key"), String::from("Value"))
        
        files.write(&hm).unwrap(); //write
    }
```

## `add` - adds content (HashMap) to file:

```rust
    use equ::Core;
    use std::collections::HashMap;

    fn main() {
        let files = core::new("safe.dat"); //new struct

        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert(String::from("Key"), String::from("Value"))
        
        files.add(&hm).unwrap(); //add
    }
```

## License:
* Project uses **Mit** license.
