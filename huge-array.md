目标： 在堆上开一个20M的内存

```
fn main() {
  {
    // 方法1： 直接用Box,但是stack overflow
    // 原因是 new里面的参数，还是先在stack 上分配
    //let b:Box<[u32;5<<20]>=Box::new([0;5<<20]);
  }

  {
    // 方法2： 先用vec,再转成Box
    // ok
    let v:Vec<u32>=vec![0;5<<20];
    let slice:Box<[u32]> = v.into_boxed_slice();
  }
}
```

为什么不直接用vec？
因为不太喜欢,vec 能拓展长度，用slice表明不能拓展长度


```
#![feature(box_syntax)]

fn main() {
    {
        // [i32;12] implement Copy
        let a = [0; 12];
        println!("{:p}", &a);
        // will copy here
        let b = a;
        // so address are different
        println!("{:p}", &b);
    }

    {
        // Box not implement Copy
        // print same address
        let a = box [0; 12];
        println!("{:p}", a);
        let b = a;
        println!("{:p}", b);
    }
}

```

