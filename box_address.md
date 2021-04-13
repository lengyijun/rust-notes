对于一般变量，获得地址非常简单：
```
let s=0;
let address:usize=&s as *const i32 as usize;
```

但是对于复杂的类型,比如Box<T>，你要区分获得地址是 Box<T> 的，还是T的.

Box<T>的地址称为 outer_address, T的地址称为 inner_address

outer_address 在 stack 上，inner_address 在 heap 上

```rust
fn main() {
    let b = Box::new([0; 10]);
    let outer_address: u64 = &b as *const Box<[i32; 10]> as u64;
    println!("{:x}", outer_address);

    let inner_address: u64 =  &*b as *const [i32;10] as u64;
    println!("{:x}", inner_address);

    // 验证inner_address
    println!("{:p}", b);
    let ptr = Box::into_raw(b);
    println!("{:p}", ptr);
}
```

```rust
fn main() {
    let x = Box::new(1);
    // stack
    println!("{:p}", &x);
    let outer_address = &x as *const Box<i32> as u64;
    println!("0x{:x}", outer_address);

    // heap
    println!("{:p}", x);
    println!("{:p}", &*x);

    let a = &*x as *const i32 as usize;
    println!("0x{:x}", a);
}
```


直接cast只能得到Box的外部地址，想要获得Box的内部地址，可以用marco来做


Box获得的地址就是里面的地址

```rust
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

fn main() {
    let b = Box::new([0; 10]);
    let address = getAddress!(b);
    println!("{}", address);
    println!("{:p}", b);

    let ptr = Box::into_raw(b);
    let address = getAddress!(ptr);
    println!("{}", address);
    println!("{:p}", ptr);
}
```
