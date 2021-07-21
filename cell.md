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

Refcell 开销比 cell 大一点

