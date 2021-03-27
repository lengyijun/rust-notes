由于Rust的特性，有些类型是不可能使用到的，以下列举一些：



| 错误写法        | 正确写法        |                                                              |
| --------------- | --------------- | ------------------------------------------------------------ |
| Arc<Cell>       | Arc<RefCell>    | 混用多线程和单线程的类型<br/>Send,Sync问题，编译不过的       |
| Cell<Box<T>>    | RefCell<Box<T>> | 可以编译，但是不能调用他的get() 方法，因为Copy trait的语速   |
| Rc<Box<T>>      | Rc<T>           | Rc<T>和Box<T>都在堆上分配，直接写成Rc<T> 就可以了            |
| Arc<Box<T>>     | Arc<T>          | Arc<T>和Box<T>都在堆上分配，直接写成Arc<T> 就可以了          |
| RefCell<Box<T>> | RefCell<T>      | Box<T> 提供可变性<br/>RefCell<T> 也提供可变性<br/>看不出有什么意义<br/>但是 Rc<RefCell<Box<T>>> 倒是可能有用处的 |
| Rc<Mutex<T>>    | Rc<RefCell<T>>  | 混用多线程和单线程的类型<br/>在单线程下使用Mutex是危险的，会造成死锁 |

