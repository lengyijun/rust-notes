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

