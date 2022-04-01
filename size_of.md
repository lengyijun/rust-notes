| DST                  | fat pointer        |
| -------------------- | ------------------ |
| 长度不确定           | 长度确定           |
| 不可以做参数、返回值 | 可以做参数、返回值 |
|                      |                    |
| [T]                  | &[T]               |
| size: not sure       | size: 16           |
|                      |                    |
| str                  | &str               |
| size: not sure       | size: 16           |
|                      |                    |



DST 对应着一个 fat pointer

   

```rust
    dbg!(mem::size_of::<&str>());
    dbg!(mem::size_of::<Rc<str>>());
    dbg!(mem::size_of::<Rc<&str>>());

    dbg!(mem::size_of::<Box<&str>>());
    dbg!(mem::size_of::<Box<str>>());
    dbg!(mem::size_of::<Rc<Box<str>>>());

    dbg!(mem::size_of::<String>());
    dbg!(mem::size_of::<Rc<String>>());

    dbg!(mem::size_of::<Rc<[u8]>>());
    dbg!(mem::size_of::<Rc<&[u8]>>());

```

