# Rc

## 什么时候需要RC？

需要一个指针，并且指针要给别的对象用的时候

如果指针只给一个人用，比如链表，那不用Rc,用Cell,RefCell,Box就行。

Rc特别用来，在单线程内，几个对象共享的指针。



比如一个图，每个Node是个struct. 在Node中要存他的邻居，就得用 Rc<Node> ，表示指向 Node的指针。

就算在单线程下，一个指针是共享的还是独有的，还是有很大区别的。


Rc和Box的区别不大，内容都在heap上，区别就是Box可变，Rc不可变。Box独有，Rc共享。


Rc<T> 会把T分配到堆上。比如我们要分配一个长数组，直觉上要这么写：

```
// ok
let a=Rc::new([0;5]);

// stackoverflow
// 5M 开在stack上太大了
let a=Rc::new([0;5<<20]);
```

但是事实上，这样写会stack overflow,因为括号里的字面量 [0;5<<20] 会首先分配到stack上,再move到heap上。

由于Rc暂时不支持这样的语法:

```
// not supported
let a:Rc<i32>=box 1;
let a:Rc<i32>=rc 1;
```

```
let a:Rc<[i32;5<<20]>=(box [0;5<<20]).into();
```

我只找到了这样的写法，缺点是引入了一个unsafe，不清楚有没有更好的写法。这种写法我是在Rc的源码里学到的：

```
#![feature(box_syntax)]

use std::rc::Rc;

fn main() {

    let b:Box<[i32;5<<20]>=box [2;5<<20];
    let a:Rc<[i32;5<<20]>;
    unsafe{
        a=Rc::from_raw(Box::leak(b));
    }
    for i in 0..1000{
        assert_eq!(a[i],2);
    }

}

```

```
let zero  = Box::<[u8; 50000000]>::new_zeroed();
```

或者用一个macro代替
```
macro_rules! rc {
    ($a:expr) => {{
        let b = box $a;
        let c;
        unsafe {
            c = Rc::from_raw(Box::leak(b));
        }
        c
    }};
}

let a = rc!([0; 5 << 20]);

```

另一种写法，就更加复杂了

```
let a: Rc<RefCell<Box<[u8; 5 << 20]>>> = Rc::new(RefCell::new(Box::new([0; 5 << 20])));
let b = a.clone();
(*b).borrow_mut()[0] = 1;
println!("{}{}", (*a).borrow()[0], (*a).borrow()[1]);
```



| Type                        | stack or heap? |
| --------------------------- | -------------- |
| [u8;1<<20]                  | stack          |
| Box<[u8;1<<20]>             | heap           |
| String                      | heap           |
| Vec<u8>                     | heap           |
| Rc<[u8;1<<20]>              | heap           |
| Rc<Vec<u8>>                 | heap           |
| Arc<[u8;1<<20]>             | heap           |
| Arc<Mutex<Box<[u8;1<<20]>>> | heap           |
| Arc<Vec<u8>>                | heap           |
| RefCell<T>                  | stack          |
| MaybeUninit<T>              | stack          |


Rc Rrc 容易循环引用，导致无法释放
Rust不保证析构函数一定会被调用。所以用Rc构造出循环引用是算在safe里面的，而不是unsafe
类似的safe,但又不是很safe的事情有 `process::exit`, 同样不会调用析构函数。

```
/// 用 `cargo run` 跑不会报错，但是用 `cargo miri run` 会报错

use std::rc::Rc;
use std::cell::RefCell;

struct S {
    x: Option<Rc<RefCell<S>>>,
}

impl Drop for S{
    fn drop(&mut self){
        println!("drop");
    }
}

fn main() {
    let a = Rc::new(RefCell::new(S { x: None }));
    let b = Rc::new(RefCell::new(S { x: None }));

    a.borrow_mut().x=Some(b.clone() );
    b.borrow_mut().x=Some(a.clone() );

}
```

## Rc 与 & 的区别
释放时机不同。Rc用计数器决定释放。& 用nll决定释放
Rc分配在heap上，& stack上


## Rc<T> 的长度
```
[src/main.rs:10] mem::size_of::<&str>() = 16
[src/main.rs:11] mem::size_of::<Rc<str>>() = 16
[src/main.rs:12] mem::size_of::<Rc<&str>>() = 8
[src/main.rs:14] mem::size_of::<Box<&str>>() = 8
[src/main.rs:15] mem::size_of::<Box<str>>() = 16
[src/main.rs:16] mem::size_of::<Rc<Box<str>>>() = 8
[src/main.rs:18] mem::size_of::<String>() = 24
[src/main.rs:19] mem::size_of::<Rc<String>>() = 8
[src/main.rs:21] mem::size_of::<Rc<[u8]>>() = 16
[src/main.rs:22] mem::size_of::<Rc<&[u8]>>() = 8
```

如果T的长度是确定的，（包括瘦指针）（和胖指针）（&str， Box<str>, &[u8], String）, `size_of Rc<T> = 8`

如果T的长度不是确定的，（DST）（str， [u8]  ）, `size_of Rc<T> = 16`

