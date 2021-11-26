|        | Souffle                                         | DDlog                                                        |
| ------ | ----------------------------------------------- | ------------------------------------------------------------ |
| 优点   | 1. nice profiler<br/>2. builtin provenance      | 1. In Rust. Easy to compile for wasm, cross-compile</br>     |
| 缺点   | 1.  链接问题<br/>2. lacks first-class data type | 1. lack disjunction (DNF)<br/>2. overhead due to incremental capacities<br/>3. bad frontend |
| 支持者 | lqd                                             | bjorn3, Dylan MacKenzie (ecstatic-morse), Domenic Quirl      |

https://github.com/vmware/differential-datalog/issues/174#issuecomment-489867074

这个链接写了souffle有，但是ddlog缺少的东西



DNF:

```
( /\ /\ ) \/ ( /\ )
```

