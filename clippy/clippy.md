看了一遍clippy，发现有些是挺容易犯的错误，有些错误背后则有很深刻的认识：

`mem_forget`
`option_option`
`path_buf_push_overwrite`
`pattern_type_mismatch`

panic unsafe 一定要文档

// 看不懂
// &Option<&T>
`ref_option_ref`

# 源码值得一看的
## 流分析
`needless_continue`
`needless_for_each`
`needless_pass_by_value`
`redundant_else` (比较简单，可以优先看一下)

## 类型分析
`redundant_allocation` (学会了 Rc，Box，&T 的判别方法)


`redundant_allocation` 只考虑了被检查的类型是函数的参数的情况，没有考虑是struct的类型的情况，所以不能尽善尽美。

比如Box<&T> 要求改成 &T, 在作函数参数的时候是合理的,因为&Box<T> 就是 &T
但是如下：
```
struct S{
 x:Box<&[u32;5<<20]>
}
```

更可能是 
```
struct S{
 x:Box<[u32;5<<20]>
}
```

而不是
```
struct S<'a>{
 x:&'a [u32;5<<20]
}
```

