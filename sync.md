| Type                   | Requirement  |
| ---------------------- | ------------ |
| u32/u8/i32 ...         | always True  |
| Box<T>                 | T: Sync      |
| Vec<T>                 | T: Sync      |
| Option<T,E>            | T: Sync      |
| Mutex<T> , RwLock<T>   | always true  |
| AtomicUsize, AtomicPtr | always true  |
|                        |              |
| RefCell<T>             | always false |
|                        |              |

 sync 指只读引用能在多线程间传递。

但是有些变量的只读引用就可以修改内容，比如Cell,RefCell

而Cell,RefCell内在实现中，没有考虑多线程的情况

所以Cell,RefCell没有实现sync