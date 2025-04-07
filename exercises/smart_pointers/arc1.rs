// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.


// 这道题目是关于 Rust 中的 多线程编程 和 线程安全智能指针 Arc<T> 的练习，目的是让你理解如何在多线程环境中共享数据，同时保证线程安全。

// 题目背景
// 问题描述：

// 给定一个包含 0..99 的 Vec<u32>，需要在 8 个线程中同时使用这些数据。
// 每个线程根据偏移量（offset）计算特定数字的和：
// 线程 0：计算 0, 8, 16, ... 的和。
// 线程 1：计算 1, 9, 17, ... 的和。
// 线程 2：计算 2, 10, 18, ... 的和。
// ...
// 线程 7：计算 7, 15, 23, ... 的和。
// 多线程中的问题：

// 多个线程需要同时访问同一个 Vec。
// Rust 的所有权规则不允许多个线程同时拥有可变引用。
// 为了解决这个问题，需要使用 线程安全的共享所有权。
// Arc<T> 的作用：

// Arc<T> 是一个 线程安全的引用计数智能指针，用于在多线程中共享数据。
// 它的全称是 Atomic Reference Counted，通过原子操作管理引用计数，保证线程安全。
// 当最后一个 Arc 实例被释放时，数据会被自动释放。

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
//     . (0..100u32)
// 范围表达式：
// (0..100) 表示一个范围，从 0 开始（包含 0），到 100 结束（不包含 100）。
// u32 是范围中数字的类型，表示无符号 32 位整数。
// 类型推断：
// Rust 会根据上下文推断范围的类型为 u32，因为后续操作需要 u32 类型。
// 2. .collect()
// 作用：
// collect() 是一个迭代器方法，用于将迭代器中的元素收集到集合中。
// 在这里，collect() 将范围 (0..100u32) 转换为一个 Vec<u32>。
// 类型推断：
// collect() 的返回类型由目标变量的类型决定。
// 这里目标变量是 numbers: Vec<_>，因此 collect() 返回一个 Vec<u32>。
// 3. Vec<_>
// 类型占位符 _：
// _ 是类型占位符，表示让编译器自动推断具体类型。
// 在这里，Vec<_> 表示一个向量，元素的具体类型由上下文推断为 u32。
// 完整含义
// (0..100u32) 创建一个范围迭代器，生成从 0 到 99 的数字。
// .collect() 将范围中的数字收集到一个 Vec<u32> 中。
// Vec<_> 表示一个向量，元素类型由编译器推断为 u32。
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
// 1. Arc<T>
// 作用：在多线程中共享数据，保证线程安全。
// 常用方法：
// Arc::new(data)：创建一个新的 Arc 实例。
// Arc::clone(&arc)：克隆一个 Arc 实例，增加引用计数。
// Arc::strong_count(&arc)：获取当前的引用计数。
// 2. 多线程
// thread::spawn：

// 创建一个新线程，执行闭包中的代码。
// 闭包中的变量需要通过 move 捕获。
// join：

// 等待线程完成，返回线程的结果。
// 3. 闭包和 move
// move：
// 将闭包捕获的变量的所有权移动到闭包中。
// 在多线程中，必须使用 move，因为线程可能在主线程结束后仍然运行。
// 4. 迭代器
// iter：
// 返回一个不可变引用的迭代器。
// filter：
// 筛选出满足条件的元素。
// sum：
// 计算迭代器中所有元素的和。
