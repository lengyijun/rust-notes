# Rc

## 什么时候需要RC？

需要一个指针，并且指针要给别的对象用的时候

如果指针只给一个人用，比如链表，那不用Rc,用Cell,RefCell,Box就行。

Rc特别用来，在单线程内，几个对象共享的指针。



比如一个图，每个Node是个struct. 在Node中要存他的邻居，就得用 Rc<Node> ，表示指向 Node的指针。

就算在单线程下，一个指针是共享的还是独有的，还是有很大区别的。





Rc不会分配到堆上

```
// stackoverflow
let a=Rc::new([0;5<<20]);

// Arc 也一样
let a=Arc::new([0;5<<20]);
```



如果又要分配到堆上，又要Rc共享指针，要这么写

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
| Rc<[u8;1<<20]>              | stack          |
| Rc<Vec<u8>>                 | heap           |
| Arc<[u8;1<<20]>             | stack          |
| Rc<Box<[u8;1<<20]>>         | heap           |
| Arc<Box<[u8;1<<20]>>        | heap           |
| Arc<Mutex<Box<[u8;1<<20]>>> | heap           |
| Arc<Vec<u8>>                | heap           |



Rc<T>, Arc<T>, Arc<Mutex<T>>  对于内存分配在stack or heap无影响，只取决于T