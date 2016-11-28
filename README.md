# binary_heap_compare
Rust binary heap with compare closure

- Cargo.toml :
```
[dependencies]
binary_heap_compare = { git = "https://github.com/antigol/binary_heap_compare" }
```

- src/main.rs :
```
extern crate binary_heap_compare;
use binary_heap_compare::BinaryHeapCompare;

let mut heap = BinaryHeapCompare::new(|x, y| x.cmp(&y));
heap.push(1);
heap.push(2);
heap.push(3);
```
