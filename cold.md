```
#[cold]
fn .....
```

传给llvm的标记
表示这个函数不太会执行到，
所以对这个函数做体积优化，而不是速度优化
