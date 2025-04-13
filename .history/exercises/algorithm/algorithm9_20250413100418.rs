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

    pub fn add(&mut self, value: T) {
        // 添加元素到数组末尾
        self.items.push(value);
        self.count += 1;
        
        // "上浮"操作 - 将新添加的元素调整到正确位置
        let mut idx = self.count;
        while idx > 1 {
            let parent = self.parent_idx(idx);
            // 如果不需要交换(即满足堆属性)，则退出循环
            if !(self.comparator)(&self.items[idx], &self.items[parent]) {
                break;
            }
            
            // 交换当前元素与父元素
            self.items.swap(idx, parent);
            idx = parent;
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
        // 获取左右子节点索引
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        // 如果右子节点存在且比左子节点更优先(根据比较器)
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            return right;
        }
        
        // 否则返回左子节点(如果左子节点不存在，由调用者确保不会调用此方法)
        left
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
        // 如果堆为空，返回None
        if self.is_empty() {
            return None;
        }
        
        // 保存堆顶元素用于返回
        let return_item = std::mem::replace(&mut self.items[1], self.items[self.count].clone());
        
        // 移除末尾元素
        self.items.pop();
        self.count -= 1;
        
        // 如果堆不为空，执行"下沉"操作调整堆
        if self.count > 0 {
            let mut idx = 1;
            // 当节点有子节点时继续调整
            while self.children_present(idx) {
                let child = self.smallest_child_idx(idx);
                
                // 如果当前节点已满足堆属性，退出循环
                if (self.comparator)(&self.items[idx], &self.items[child]) {
                    break;
                }
                
                // 交换当前节点与子节点
                self.items.swap(idx, child);
                idx = child;
            }
        }
        
        Some(return_item)
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