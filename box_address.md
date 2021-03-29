对于一般变量，获得地址非常简单：
```
let s=0;
let address:usize=&s as *const i32 as usize;
```

但是对于复杂的类型,比如Box<T>，你要区分获得地址是 Box<T> 的，还是T的.

Box<T>的地址称为 outer_address, T的地址称为 inner_address

```rust
macro_rules! getAddress {
    ($a:expr) => {{
        let y = format!("{:p}", $a);
        let address = y.trim_start_matches("0x");
        let z: u64 = u64::from_str_radix(&address, 16).unwrap();
        z
    }};
}

fn main() {
    let b = Box::new([0; 10]);
    let outer_address=&b as *const Box<[i32;10]> as u64;
    let inner_address = getAddress!(b);
    
    println!("{:x}", outer_address);
    println!("{:x}", inner_address);
    println!("{:p}", b);

	// 验证inner_address
    let ptr = Box::into_raw(b);
    let address = getAddress!(ptr);
    println!("{:x}", address);
    println!("{:p}", ptr);
}
```



直接cast只能得到Box的外部地址，想要获得Box的内部地址，我只想出来用marco来做