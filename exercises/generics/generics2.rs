// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

struct Wrapper<T> {
    value: T,
}

//implの直後にTを宣言しなければならないことに注意してください。こうすることで、型Point<T>にメソッドを実装していることを指定するために、Tを使用することができます。 implの後にTをジェネリックな型として宣言することで、コンパイラは、Pointの山カッコ内の型が、 具体的な型ではなくジェネリックな型であることを認識できるのです。 from The Rust Programming Language
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
