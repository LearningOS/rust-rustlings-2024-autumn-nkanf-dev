/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + Copy,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Copy,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn swap(&mut self, idx1: usize, idx2: usize) {
        let temp = self.items[idx1].clone();
        self.items[idx1] = self.items[idx2];
        self.items[idx2] = temp;
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.count == 1 {
            self.items.pop();C
            self.items.push(value);
            return;
        }
        self.items.push(value);
        self.sift_up(self.count - 1);
    }

    fn sift_up(&mut self, mut idx: usize) {
        loop {
            if idx == 0 {
                break;
            } else {
                let parent_idx = self.parent_idx(idx);
                if (self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                    break;
                } else {
                    let temp = self.items[parent_idx].clone();
                    self.items[parent_idx] = self.items[idx];
                    self.items[idx] = temp;
                }
                idx = parent_idx;
            }
        }
    }

    pub fn pop_top(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.swap(0, self.count - 1);
        let ret = self.items.pop();
        match ret {
            None => None,
            Some(_) => {
                self.count -= 1;
                self.sift_down(0);
                ret
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        loop {
            if idx == self.count - 1 || idx == self.count - 2 {
                if idx == 0 && self.count == 2 {
                    if (self.comparator)(&self.items[idx], &self.items[1]) {
                        break;
                    } else {
                        self.swap(1, idx);
                    }
                    idx = 1;
                } else {
                    break;
                }
            } else {
                let mut m_idx = self.left_child_idx(idx);
                if (self.comparator)(&self.items[self.right_child_idx(idx)], &self.items[m_idx]) {
                    m_idx = self.right_child_idx(idx);
                }
                if (self.comparator)(&self.items[idx], &self.items[m_idx]) {
                    break;
                } else {
                    let temp = self.items[idx].clone();
                    self.items[idx] = self.items[m_idx];
                    self.items[m_idx] = temp;
                }
                idx = m_idx;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Copy,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(e) = self.pop_top() {
            Some(e)
        } else {
            None
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        print!("{:?}", heap);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        print!("{:?}", heap);
        assert_eq!(heap.next(), Some(9));
        print!("{:?}", heap);
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
