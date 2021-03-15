由于Rust的特性，有些类型是不可能使用到的，以下列举一些：

Arc<Cell>
Arc<RefCell>
Send,Sync问题，编译不过的

Cell<Box<T>>
可以编译，但是不能调用他的get() 方法，因为Copy的问题，所以基本没有意义

Rc<Box<T>>
Arc<Box<T>>
Rc和Box都在堆上分配，直接写成Rc<T> 就可以了

Rc<Mutex<T>>
使用Rc表面是单线程。单线程下，不能使用Mutex。
应该用Rc<RefCell<T>>


