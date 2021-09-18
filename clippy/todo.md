TODO 
/0 检查
数据流分析

TODO 
`&PathBuf as &Path` 这个lint不会自动提示说 `&**PathBuf`
提示不够好，不能自动修复
能查出deref后的类型吗？

TODO
自动 COW 提示

TODO 
能不能查出可能导致死锁的mutex使用？
https://cseweb.ucsd.edu/~yiying/RustStudy-PLDI20.pdf

LATER
```
&*(&*x) 
```
的自动修复有很大问题



