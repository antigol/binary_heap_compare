use std::cmp::Ordering;

#[allow(dead_code)]
pub struct BinaryHeapCompare<T, F> {
    array: Vec<T>,
    compare: F,
}

#[allow(dead_code)]
impl<T, F> BinaryHeapCompare<T, F>
    where F: Fn(&T, &T) -> Ordering
{
    pub fn new(cmp: F) -> BinaryHeapCompare<T, F> {
        BinaryHeapCompare {
            array: Vec::new(),
            compare: cmp,
        }
    }

    pub fn from_vec(xs: Vec<T>, cmp: F) -> BinaryHeapCompare<T, F> {
        let mut r = BinaryHeapCompare {
            array: xs,
            compare: cmp,
        };

        for i in (0..r.array.len()).rev() {
            r.shift_down(i);
        }

        r
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn push(&mut self, x: T) {
        self.array.push(x);
        let i = self.array.len() - 1;
        self.shift_up(i);
    }

    pub fn peek(&mut self) -> Option<T> {
        if self.array.is_empty() {
            None
        } else {
            let r = self.array.swap_remove(0);
            self.shift_down(0);
            Some(r)
        }
    }

    fn shift_up(&mut self, mut i: usize) {
        while i > 0 {
            let p = i / 2;
            if (self.compare)(&self.array[i], &self.array[p]) == Ordering::Greater {
                self.array.swap(i, p);
                i = p;
            } else {
                return;
            }
        }
    }

    fn shift_down(&mut self, mut i: usize) {
        //      0
        // 1       2
        // 3   4   5   6
        // 7 8 9
        //
        loop {
            let j = 2 * i + 1;
            let k = 2 * i + 2;

            if j >= self.array.len() {
                return;
            }

            if k >= self.array.len() {
                if (self.compare)(&self.array[i], &self.array[j]) == Ordering::Less {
                    self.array.swap(i, j);
                    i = j;
                } else {
                    return;
                }
            } else {
                if (self.compare)(&self.array[i], &self.array[j]) == Ordering::Less {
                    if (self.compare)(&self.array[j], &self.array[k]) == Ordering::Less {
                        // i < j < k
                        self.array.swap(i, k);
                        i = k;
                    } else {
                        // i < k < j
                        // k < i < j
                        self.array.swap(i, j);
                        i = j;
                    }
                } else {
                    if (self.compare)(&self.array[i], &self.array[k]) == Ordering::Less {
                        // j < i < k
                        self.array.swap(i, k);
                        i = k;
                    } else {
                        // j < k < i # ok
                        // k < j < i # ok
                        return;
                    }
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut h = BinaryHeapCompare::new(|x: &i32, y: &i32| x.cmp(y));
    h.push(1);
    h.push(2);
    h.push(3);
    h.push(0);
    assert_eq!(h.peek(), Some(3));
    assert_eq!(h.peek(), Some(2));
    assert_eq!(h.peek(), Some(1));
    assert_eq!(h.peek(), Some(0));
    assert_eq!(h.peek(), None);
}

#[test]
fn it_works_from_vec() {
    let mut h = BinaryHeapCompare::from_vec(vec![1,3,5,2], |x: &i32, y: &i32| x.cmp(y));
    assert_eq!(h.peek(), Some(5));
    assert_eq!(h.peek(), Some(3));
    assert_eq!(h.peek(), Some(2));
    assert_eq!(h.peek(), Some(1));
    assert_eq!(h.peek(), None);
}

#[test]
fn it_works_empty() {
    let h = BinaryHeapCompare::new(|x: &i32, y: &i32| x.cmp(y));
    assert!(h.is_empty())
}
