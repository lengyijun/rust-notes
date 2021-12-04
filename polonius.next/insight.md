# 对指针/引用的理解

在mir中，典型的指令有三种：

```
// Borrow
_2 = &_1

// Move
_2 = move _1

// Copy
_2 = _1

// Call
Vec::push(_2, _1)
```

Borrow中， _2 与 _1 有强关联。
因为 _1 一旦失效（ _1 += 1 ）， _2 也马上失效。

Move 中，
_1 是没有Copy的。
Move之后，_1 也就失效了。所以此情况不用讨论。

Copy 是最难的。
如果是 u32 这种Copy不会有什么影响。
但是 &T 就会非常麻烦。
这里会生成 subset Origin(_1) Origin(_2) 的关系。
这个关系会向下传递（not always,在特定的构造里会发生）。
到未来某一刻，Origin(_1) 里塞了其他东西，但已经和 _2 没有了关系，
但是 subset 的存在导致 _2 里塞了奇怪的东西。
所以，在恰当的时候，必须止住 subset 关系的传递。


Call 的情况千变万化。
首先根据函数的签名要生成 subset 关系。
这个 subset 关系也会像之前一样传到不合适的位置。
同样需要 clear 来止住。

# 强关联与弱关联
强关联情况，father 持有loan。一个 loan_invalidate 就解决了。这个polonius已经做的很好了

弱关联情况，father 不持有loan。 father 和 child 之间只有subset关系。
polonius处理的不好，需要 polonius.next 

|        |                     |        |      |
| :----- | :------------------ | :----- | :--- |
| Borrow | \_1 = &\_2          | 强关联 |      |
| Move   | \_1 = move \_2      | 弱关联 |      |
| Copy   | \_1 = \_2           | 弱关联 |      |
| Call   | Vec::push(\_1, \_2) | 弱关联 |      |
|        |                     |        |      |


# 如何止住？ 如何clear？
有些Call要生成 clear_origin : 

```
use std::mem;

fn main() {
  let a = 1;
  let mut b = 2;

  let mut aa = &a;

  let x = &*aa;

  mem::replace(&mut aa, &b);  // should generate clear_origin ?
  b += 1;

  x;
}
```

这个replace 修改了 aa 指向，终止了 aa 与 x 关系。

以下语句都应该生成 clear_origin Origin(_1): 

```
_1 = _2
_1 = move _2
_1 = call()
```

|                      |                                        |                                          |
| -------------------- | -------------------------------------- | ---------------------------------------- |
| Vec::push(\_1, \_2)  | clear_origin Origin(_1)                |                                          |
| mem::replace(_1, _2) | clear_origin _1 指向的东西的所有Origin | 如何表达？                               |
|                      |                                        | 所有call内部改变指向的，都要clear origin |

