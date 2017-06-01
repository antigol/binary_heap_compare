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

let mut heap = BinaryHeapCompare::new(|x: &i32, y: &i32| x.cmp(&y));
heap.push(1);
heap.push(2);
heap.push(3);
assert!(heap.pop() == Some(3));
assert!(heap.peek() == Some(&2));
```
## Documentation

Methods
- `new(cmp: F) -> BinaryHeapCompare<T, F>`
- `fn from_vec(xs: Vec<T>, cmp: F) -> BinaryHeapCompare<T, F>`
- `fn is_empty(&self) -> bool`
- `fn len(&self) -> usize`
- `fn clear(&mut self)`
- `fn push(&mut self, x: T)`
- `fn pop(&mut self) -> Option<T>` remove and return max
- `fn peek(&self) -> Option<&T>` return ref to max
- `fn retain<G: FnMut(&T) -> bool>(&mut self, mut f: G)` remove elements when f return false

Traits
- `fmt::Debug`
- `Clone`
- `Default`
