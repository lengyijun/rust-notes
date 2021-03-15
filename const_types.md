# const的类型
Atomic
String
u8
MaybeUninit



# 不是const的类型
Arc(why?)
Mutex(why?)
RwLock

# 所以比较安全的全局变量写法（线程安全的）
MaybeUninit<Mutex<T>>
MaybeUninit<RwLock<T>>

Option<Mutex<T>> //有点奇怪
Option<RwLock<T>> //有点奇怪

AtomicI8

# 可以通过编译，但是糟糕的写法（线程不安全）
u8

