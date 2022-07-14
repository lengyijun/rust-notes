```
fn main() {
    let mut x = 0u32;
    let y = &mut x as *mut u32;
    x = 1;
    y;   // no error
    *y;  // error
}
```

只复制不合理的指针不会报错
访问不合理的指针才报错

