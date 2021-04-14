每一个 &T 都相当于一个指针，我们可以直接从 &T 中获得指针
如果只需要打印出来比较方便，但是要在程序中获得这个u64 需要一些转换

```rust
fn main() {
    let a:[u8;100]=[0;100];
    let b:&u8=&a[0];
    unsafe{
        let address: u64  = std::mem::transmute(b);
        println!("{:x}",address);
    }
    let c:&u8=&a[1];
    unsafe{
        let address: u64  = std::mem::transmute(c);
        println!("{:x}",address);
    }
}
```


对于胖指针，不是一个u64,而是两个u64. 第二个是长度，不需要关注

```rust
fn main() {
    let a:[u8;100]=[0;100];
    let b:&[u8]=&a;
    unsafe{
        let (address,_): (u64,u64)  = std::mem::transmute(b);
        println!("{:x}",address);
    }
}
```

还有个例子

```rust
fn main() {
     let a=0;
     let b=&a;
     let c=&&a;
     let d=&&&a;
     println!("{:p}",b);
     println!("{:p}",c);
     println!("{:p}",d);
}
```

还有种方法快速获得裸指针

```
#![feature(raw_ref_op)]

let a=12;
let ptr:*const i32 = &raw const a;
```
