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
    
    pub fn pop(&mut self) -> Result<T, &str> {
        // 如果 q1 为空，无法弹出元素
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
        
        // 将 q1 中除了最后一个元素之外的所有元素移到 q2
        let size = self.q1.size();
        for _ in 0..size-1 {
            if let Ok(val) = self.q1.dequeue() {
                self.q2.enqueue(val);
            }
        }
        
        // 弹出 q1 中的最后一个元素（即栈顶元素）
        let result = self.q1.dequeue();
        
        // 交换 q1 和 q2，使 q1 始终是存储元素的队列
        std::mem::swap(&mut self.q1, &mut self.q2);
        
        result
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
		Err("Stack is empty")
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        true
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