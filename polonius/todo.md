# return a struct
https://github.com/rust-lang/rust/issues/70797

  related to eager drop

  If the mir polonius works on is equiped with eager drop, this can be fixed.


- HRKB

# initialization
- move error

- 规则有冗余
`path_assigned_at` is not needed, it can be replaced with  `path_maybe_initialized_on_exit`

# liveness
`origin_live_on_entry` 不计算 `loan_issued_at` ? (Solved)
  不计算，只有使用了才计算。典型错误。

# other
- replace `universal_region` to `placeholder`

- 自动化的从rs开始编译测试，而不用在git上存这么多 nll-facts

- `drop_of_var_derefs_origin` 的标准是什么？是不是只要自定义的drop都会生成这一条，和drop具体的实现并没有关系？

- cheatsheet 里的TODO

# 是不是sensitive？
```
fn foo1(){bar(...);}
fn foo2(){bar(...);}
fn bar(..){}
```

bar在foo1 和 foo2 里要分析两次吗
placeholder

- Vec::push(&x)
  到底是怎么生成规则的？(Solved)
  FnSig


- 有没有可能一个类型里没有lifetime,却实际上需要lifetime?

`&*` 为什么会搞出一个 `path_assigned_at_base`?
 ~/polonius/inputs/smoke-test/nll-facts/foo

# `datafrog_opt`
怎么想出来的?
如何证明和naive是等价的？

# const fn
const fn 和polonius什么关系？

