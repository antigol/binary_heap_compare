use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Default)]
pub struct BinaryHeapCompare<T, F> {
    array: Vec<T>,
    compare: F,
}

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

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn clear(&mut self) {
        self.array.clear();
    }

    pub fn push(&mut self, x: T) {
        self.array.push(x);
        let i = self.array.len() - 1;
        self.shift_up(i);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.array.is_empty() {
            None
        } else {
            let r = self.array.swap_remove(0);
            self.shift_down(0);
            Some(r)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.array.is_empty() {
            None
        } else {
            Some(&self.array[0])
        }
    }

    pub fn retain<G: FnMut(&T) -> bool>(&mut self, mut f: G) {
        for i in (0..self.array.len()).rev() {
            if !f(&self.array[i]) {
                self.array.swap_remove(i);
                self.shift_down(i);
            }
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

impl<T, F> fmt::Debug for BinaryHeapCompare<T, F>
    where F: Fn(&T, &T) -> Ordering,
          T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}
