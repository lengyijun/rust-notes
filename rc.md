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
| Rc<Box<[u8;1<<20]>>         | heap           |
| Arc<Box<[u8;1<<20]>>        | heap           |
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
