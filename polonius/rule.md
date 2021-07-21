在liveness 中， 没有 path 的rule
在borrow check中，没有 path 和 variable 的rule

loan不是孤立的存在的。一个Loan 一生出来就有一个Origin包住他了
不会有 (Loan,Loan) 这种规则

contain: origin 与 loan 的关系
subset： origin 与 origin 的关系
`_base`: 表示只有直接的边相连，没有间接边相连

# `_#0r` `_#1r` 总是出现在 placeholder 和 `universal_region` 里面，他们是什么？
`_#0r`:`_#1r`
并且他们对应的placeholder是特别的，最后统一加上去的，不会在别的地方用到

origin 只记录live,不记录死亡。 所以origin总是增加的。
`loan_invalidated_at` 用来做最后的错误判断，并不参与规则的扩散

var 和 origin 的关系是确定的
origin 和 origin 的关系是动态的。origin 和 loan 的关系也是动态的


```
drop(a);
drop(a.0);  // move error
```

```
drop(a.0);  
drop(a);   // move error
```

```
drop(a.0);
a=S{}    // ok
```

```
drop(a);
a.0=S{}   // partial move error
```


整个函数的入口是 `path_moved_at` ，给每个path一个空值，这样可以检查到这些空值会流到什么地方，检查出未定义的变量（move error）
在polonius中，使用未初始化的变成是 move error

"drop live" is a superset of "use live" 

`partial initialization` 可能会在polonius中被支持


`move_error` 与 `&` 无关
`error` 与 `&` 有关

-------------------

```
_1=& _2;
```
emit:
```
loan_issued_at()
path_accessed_at()
path_assigned_at()
```
`_1` 关联的Origin在这一刻不是live的

-------------------

`origin_live_on_entry(origin,point)` 中的origin,一定来自于 ` use_of_var_derefs_origin` 或者 `drop_of_var_derefs_origin`

