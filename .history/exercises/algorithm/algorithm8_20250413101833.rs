/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

// 修改为符合 Rust 命名规范的名称
pub struct myStack<T>
{
    q1: Queue<T>,
    q2: Queue<T>
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new()
        }
    }
    
    pub fn push(&mut self, elem: T) {
        // 始终将新元素添加到 q1
        self.q1.enqueue(elem);
    }
    
    pub fn pop(&mut self) -> Result<T, &'static str> {
        // 如果 q1 为空，无法弹出元素
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
        
        // 将 q1 中除了最后一个元素之外的所有元素移到 q2
        let size = self.q1.size();
        for _ in 0..size-1 {
            // 使用模式匹配直接处理结果，避免保留对 self.q1 的引用
            match self.q1.dequeue() {
                Ok(val) => self.q2.enqueue(val),
                Err(_) => return Err("Unexpected error"), // 这种情况理论上不会发生
            }
        }
        
        // 直接处理最后一个元素的结果，而不保存可能含有引用的 Result
        let value = match self.q1.dequeue() {
            Ok(val) => val,
            Err(_) => return Err("Unexpected error"),
        };
        
        // 现在可以安全地交换队列
        std::mem::swap(&mut self.q1, &mut self.q2);
        
        Ok(value)
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}