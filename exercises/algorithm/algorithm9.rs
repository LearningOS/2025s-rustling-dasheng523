/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
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

    fn heapify_up(&mut self, idx: usize) {
        if idx == 1 {
            return;
        }
        let mut idx = idx;
        let current = &self.items[idx];
        let parent_idx = self.parent_idx(idx);
        let parent_num = &self.items[parent_idx];
        let rs = (self.comparator)(current, parent_num);
        if rs {
            self.items.swap(idx, parent_idx);
            self.heapify_up(parent_idx);
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        println!("heapify_down: {}", idx);
        let current = &self.items[idx];
        if !self.children_present(idx) {
            return;
        }
        let left_child_idx = self.left_child_idx(idx);
        let right_child_idx = self.right_child_idx(idx);
        println!("index: {} {} {}", left_child_idx, right_child_idx, idx);
        let left_child_num = &self.items[left_child_idx];

        if right_child_idx >= self.count {
            let rs = (self.comparator)(left_child_num, current);
            if rs {
                self.items.swap(idx, left_child_idx);
                self.heapify_down(left_child_idx);
            }
            return;
        }

        let right_child_num = &self.items[right_child_idx];
        let rs = (self.comparator)(left_child_num, right_child_num);
        if rs { 
            let rs = (self.comparator)(left_child_num, current);
            if rs {
                self.items.swap(idx, left_child_idx);
                self.heapify_down(left_child_idx);
            }
        } else { 
            let rs = (self.comparator)(right_child_num, current);
            if rs {
                self.items.swap(idx, right_child_idx);
                self.heapify_down(right_child_idx);
            }
        };
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count);
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
        1
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 如果堆为空，返回 None
        if self.count == 0 {
            return None;
        }

        // 交换堆顶和堆尾元素
        self.items.swap(1, self.count);

        // 取出堆尾元素
        let item = self.items.pop();
        self.count -= 1;

        // 堆化
        self.heapify_down(1);

        item
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}