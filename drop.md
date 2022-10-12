有两个drop 
1. mem::drop
一个空函数

2. Drop::drop
不能手工调用 `<T as Drop>::drop(&mut t)`
在mir中编译器自动插入


只有Drop的变量才会被调用drop

## mir 中的drop

正如 [nonzeroing](https://rust-lang.github.io/rfcs/0320-nonzeroing-dynamic-drop.html)
中所写的，所有Drop的变量都会有一个bool与之对应。

在mir中可以看到： SimplifyCfg-elaborate-drops.after.dot 

## drop flag

https://doc.rust-lang.org/nomicon/drop-flags.html

虽说rust 中静态的成分多，不过还是有动态的

https://www.thecodedmessage.com/posts/raii/
这里也有提到

