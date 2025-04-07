// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.



// 递归类型的问题：

// Rust 在编译时需要知道每个类型的大小（size）。
// 对于递归类型（如 List），因为它包含自身的实例，编译器无法确定它的大小。
// 解决方法是使用 Box，它是一个智能指针，存储在堆上，大小是固定的（指针大小）。
// Cons List：

// Cons List 是一种常见的递归数据结构，常用于函数式编程语言。
// 它由两部分组成：
// 当前节点的值。
// 指向下一个节点的指针（或表示结束的 Nil）。
#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
   List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
