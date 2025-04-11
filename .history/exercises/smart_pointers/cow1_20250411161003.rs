// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


// 这道题目是关于 Rust 中的 Cow（Clone-On-Write）智能指针 的练习，目的是让你理解 Cow 的工作原理以及如何在需要时高效地处理数据的克隆和共享。

// Cow 的背景知识
// Cow 是什么？

// Cow 是 Rust 中的一个智能指针，表示 "写时克隆"（Clone-On-Write）。
// 它可以封装 借用的数据（Borrowed）或 拥有的数据（Owned）。
// 当需要对数据进行修改时，如果数据是借用的，Cow 会克隆数据并将其转换为拥有的数据。
// Cow 的用途

// 提供对数据的高效访问：如果数据不需要修改，可以直接使用借用的数据。
// 延迟克隆：只有在需要修改时才会克隆数据，避免不必要的性能开销。
// Cow 的两种状态

// Cow::Borrowed：表示数据是借用的，未发生克隆。
// Cow::Owned：表示数据是拥有的，可能是原始数据，也可能是克隆后的数据。
// 题目要求
// 理解 Cow 的行为：

// 如果数据需要修改且是借用的，Cow 会克隆数据并切换到 Owned 状态。
// 如果数据不需要修改，Cow 保持 Borrowed 状态。
// 如果数据本身是拥有的，即使不需要修改，Cow 仍然是 Owned 状态。
// 修复测试用例：

// 根据 Cow 的状态（Borrowed 或 Owned），在 match 分支中填入正确的逻辑。
use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}
// 参数：
// input 是一个可变引用，类型为 Cow<'b, [i32]>。
// 它可以封装一个借用的切片（&[i32]）或一个拥有的向量（Vec<i32>）。
// 逻辑：
// 遍历 input 中的每个元素。
// 如果元素小于 0，则调用 to_mut()：
// 如果 input 是 Borrowed，to_mut() 会克隆数据并切换到 Owned。
// 如果 input 已经是 Owned，to_mut() 直接返回可变引用。
// 将负数转换为正数。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]); // Borrowed slice
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]); // Borrowed slice
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice); // Owned vector
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice); // Owned vector
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
