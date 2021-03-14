| Type        | 条件                                    |
| ----------- | --------------------------------------- |
| Arc<T>      | T: Send                                 |
| Box<T>      | T: Send                                 |
| RefCell<T>  | T: Send                                 |
| Cell<T>     | T: Send                                 |
| Option<T,E> | T: Send , E:Send                        |
| Mutex<T>    | 无约束，因为Mutex自己保证的Send的安全性 |
| Rc<T>       | never                                   |
|             |                                         |
|             |                                         |



