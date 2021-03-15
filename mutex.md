先说结论，在Rust中，Mutex只有两种用处。
	1. 与Arc一起出现. Arc<Mutex<T>> 处理多线程的变量读写
  2. MaybeUninit<Mutex<T>> 用来做全局变量

还有另一个结论：

  1. 单线程中，永远不用Mutex

这还意味着，如果一个struct只在单线程中使用，那么他的每一个字段都不能是Mutex


首先在单线程中使用Mutex是危险的，因为没有unsafe和&mut的条件下，就能构造一个死锁：


```rust
use std::sync::Mutex;

// deadlock example
fn main() {
    let a=Mutex::new(0);
    let b=a.lock().unwrap();
    let c=a.lock().unwrap();
    println!("{}",*b);
}

```

这还意味着 单线程中Mutex没有意义。一定能找到方法替代，比如可变引用，Cell,RefCell.


再说Mutex做全局变量的写法。

```
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
use std::thread;

static mut M: MaybeUninit<Mutex<u32>> = MaybeUninit::uninit();

fn main() {
    thread::spawn(move || unsafe {
        let m = M.as_mut_ptr();
        m.write(Mutex::new(3));
    })
    .join()
    .expect("thread::spawn failed");

    unsafe {
        let m = M.as_mut_ptr();
        m.write(Mutex::new(2));
    }

    unsafe {
        assert_eq!(*M.as_ptr().read().lock().unwrap(), 2);
    }
}

```

之所以Mutex外面一定要包一层 MaybeUninit ,是因为Mutex不是const的。全局变量必须是const的。
而MayUninit是const的。

至于为什么Mutex不是const的，

https://github.com/rust-lang/rust/issues/66806

简单的说Mutex依赖os. 在有些os上，Mutex并不容易const. 为了不失一般性，所以没有加。
但值得一提的是，AtomicI8 是const的，可以方便的用作全局变量。


最后一种Mutex包在Arc里面，是最常见的用法。方便clone() 之后move到另一个线程.

Mutex 还可以用在struct中.不过和前面一样，如果struct只在单线程使用，没必要用mutex.
如果要多线程使用，struct外面还是要包一层Arc,本质上还是 `Arc<Mutex<T>>`


本质上说，Rust中的Mutex的使用方法和golang中的Mutex使用起来是很不一样的。在Rust中，你搞不清楚需不需要Mutex的时候，可以先不加，等到报错的时候再补上也来得及。








多线程中，假设你需要在多个线程一个共享变量，你把他类型写成 Mutex<T>.然后在另一个线程要用这个变量，你总是要传个引用吧。然后问题来了，跨线程不能传引用，因为生命周期未知。

