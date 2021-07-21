总体步骤：
1. 用自定义的polonius编译rustc
2. 用自定义的rustc 编译测试的 .rs 文件, 生成中间文件
3. 把中间文件反过来给polonius测试


## 用自定义的polonius编译rustc
具体的说，rustc默认使用的polonius太旧了，我们都不爱用。一般都要用自己的。
polonius目前只替代了borrow checker中的部分功能。
比如move error目前还是不由polonius报的 ，尽管polonius能够检查出这些错误。

所以用polonius编译的rustc,就算利用了polonius检查出了move error,也不会报。
所以要用polonius自己测试.

修改 rust 里 Cargo.toml 的路径。
只需要rustc,所以这样编译：
```
# ~/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
./x.py build -i compiler/rustc
```

## 用自定义的rustc 编译测试的 .rs 文件, 生成中间文件
一般不用cargo,直接用rustc
```
~/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Znll-facts -Zdump-mir="all" -Zdump-mir-graphviz  main.rs 
```

其中 nll-facts 目录是给polonius 用的，`mir_dump` 是用来对照的看的。
rustc会生成多个不同状态的mir, https://rustc-dev-guide.rust-lang.org/mir/passes.html
`main.main.-------.nll.0.dot` 似乎是有用的那个
当有编译错误的时候，无法生成最终版本的mir,只能用看中间版本的mir。

## 把中间文件反过来给polonius测试
看polonius的Readme

```
cargo +nightly run --release -- -a DatafrogOpt --show-tuples -v --graphviz-file a.dot /path/to/nll-facts/main
```

注意，如果 nll-facts 太大，会OOM

## origin 的包含关系
```
# can generate `origin` information in .dot format
rustc -Zdump-mir="all" -Zdump-mir-dataflow main.rs
tod qinx.main.-------.nll.0.regioncx.all.dot
```
生成origin信息.
这个信息的文字版，在 `main.main.-------.nll.0.mir` 的开头也有
`main.main.-------.nll.0.mir` 的开头列表必须看一遍

