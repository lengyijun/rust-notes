对于一般变量，获得地址非常简单：
```
let s=0;
let address:usize=&s as *const i32 as usize;
```

但是对于复杂的类型,比如Box<T>，你要区分获得地址是 Box<T> 的，还是T的.

Box<T>的地址称为 outer_address, T的地址称为 inner_address

```rust
fn main() {
    let b = Box::new([0; 10]);
    let outer_address: u64 = &b as *const Box<[i32; 10]> as u64;
    // 从Rust源码中借鉴过来的
    // impl Pointer for Box
    let inner_address: u64 = {
        let c: *const [i32; 10] = &**(&b);
        c as u64
    };

    println!("{:x}", outer_address);
    println!("{:x}", inner_address);
    println!("{:p}", b);

    // 验证inner_address
    let ptr = Box::into_raw(b);
    println!("{:p}", ptr);
}
```



直接cast只能得到Box的外部地址，想要获得Box的内部地址，我只想出来用marco来做


```
// 一个没什么用的macro,虽然也可以用
// 把 {:p} 打印出来的，转成u64
macro_rules! getAddress {
    ($a:expr) => {{
        let y = format!("{:p}", $a);
        let address = y.trim_start_matches("0x");
        let z: u64 = u64::from_str_radix(&address, 16).unwrap();
        z
    }};
}
```
