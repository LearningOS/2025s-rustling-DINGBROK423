// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


// mem 是 Rust 标准库中的一个重要模块，全名为 std::mem，专门提供与内存操作相关的实用函数。这个模块帮助开发者处理底层内存管理和操作
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }
    if let Some(())= my_option {  }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let  mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);
// 
    let mut value_a = 45;
    let mut value_b = 66;
    // let tmp= value_a;
    // // Let's swap these two!
    // value_a = value_b;
    // value_b = tmp;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
