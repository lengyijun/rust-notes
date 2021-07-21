```
# 会生成 StorageDead(_1); StorageLive(_1);                  
cargo rustc  --release -- --emit mir

# 不会生成 StorageDead(_1); StorageLive(_1);                  
cargo rustc  -- --emit mir
```

`StorageDead(_1); StorageLive(_1); `  是用来给LLVM 分配stack用的，
如果两个变量的生命周期没有重叠，那么他们可以分配在同一个stack 地址上



`StorageDead(_1); StorageLive(_1);` 是非常有用的东西，因为debug中没有加这些，导致LLVM不知道变量的lifetime,有些用不到的变量无法优化掉，性能就会不好。



比如我开一个用不到的stack上的大元素，debug模式下，这个变量会分配，release下不会分配，

所以在debug下，运行会报错，但是release模式下不会报错


mir 中看不到是不是unsafe。unsafe只能在hir中判断

