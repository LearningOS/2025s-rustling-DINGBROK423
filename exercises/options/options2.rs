// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        let word = optional_target.unwrap() ;
        {
            assert_eq!(word,target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
       while let Some(Some(integer)) = optional_integers.pop()
        {
            assert_eq!(integer,cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

// 标准库中，`pop()`返回的是`Option<T>`，其中`T`是向量的元素类型。在这里，向量的元素类型是`Option<i8>`，所以`pop()`的返回值应该是`Option<Option<i8>>`。也就是说，外层的`Option`表示向量是否为空，如果向量非空，返回的是`Some(元素)`，这里的元素本身又是一个`Option<i8>`，所以内层的`Option`是元素本身的值。
