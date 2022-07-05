Cell类似与 `int *`  in cpp

Cell和RefCell内在都是用 UnsafeCell
UnsafeCell<T> 和 T 其实差不多，不过是被编译器照顾的对象 [lang = 'unsafecell']

如果说Cell专为Copy实现是不准确的。Cell有两套api,一套是copy的，一套是default的

```
# copy
// impl<T: Copy> Cell<T>
pub fn get(&self) -> T

// impl<T> Cell<T>
pub fn set(&self, val: T)
```

```
# default
// impl<T: Default> Cell<T>
pub fn take(&self) -> T

// impl<T> Cell<T>
pub fn set(&self, val: T)

// impl<T> Cell<T>
pub fn replace(&self, val: T) -> T
```

Refcell 开销比 Cell 大一点

Cell  线程不安全 

Cell  是转化 mut 和 非mut 之间的魔法
如果签名要求非mut,但是你又非要mut,那么Cell可能就要用到 

shared references may not be used to perform mutation except when the mutation happens łinsidež an UnsafeCell.

Cell 不提供内部的指针
RefCell 提供内部的指针


Cell<u8> 和 u8 几乎一样
&Cell<u8> 和 &u8 很不一样

