在程序静态分析中，在指针分析中，函数内的指针分析和跨函数的指针分析完全就是两回事。
于是我在想，lifetime分析，在函数内的分析，和跨函数的lifetime分析，也是天壤之别。

函数内的lifetime分析我就不多说了，应该都看得懂

跨函数的lifetime分析让我非常困扰。
我们打算回答以下的问题：
1. Rust中lifetime分析是context free还是context sensitive的？

解释以下context free和context sensitive：
如果Rust中lifetime分析是context free的，那意味着 longest只需要分析一遍.

无论有多少个其他函数调用longest，在遇到longest的时候，只需要检查以下longest的规则影响，不用把实际参数带入longest计算一遍。 

如果是context sensitive的，
那每次有别的函数调用 longest，都需要带入参数计算一遍。
这会有很大的开销，但是效果会更好（在别的语言上如此）

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```


https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-borrow-checker

以我对rust的理解，应该是context free的。具体分析如下：


```
1: fn main() {
2:     let string1 = String::from("long string is long");
3:     let result;
4:     {
5:         let string2 = String::from("xyz");
6:         result = longest(string1.as_str(), string2.as_str());
7:     }
8:     println!("The longest string is {}", result);
9: }
```

string2 的lifetime是 5，6
string1 的lifetime是 2,3,4,5，6,7,8

'a 是 min(*string1's lifetime,string2's lifetime)
'a = string2 lifetime = {5,6}

根据函数签名， result的lifetiem是 {5,6}

但是result要在第8行用到，所以会报错。


如果用polonius分析，也能得到相同的报错，但是分析过程不一样：

```
1: fn main() {
2:     let string1 = String::from("long string is long");
3:     let result;
4:     {
5:         let string2 = String::from("xyz");
6:         result = longest(string1.as_str(),   //L1
                             string2.as_str()); //L2
7:     }
8:     println!("The longest string is {}", result);
9: }
```

string1.as_str 我们标记成Loan 1(L1)
string2.as_str 我们标记成Loan 2(L2)

L1 outlive 'a
L2 outlive 'a

L1: 'a
L2: 'a


L1 属于 'a
L2 属于 'a

'a = {L1,L2}

所以result的lifetime是 {L1,L2}
result在第8行要使用的时候，L2已经失效，所以会报错

从longest这个例子，我们也看出为什么返回值有lifetime的函数，必须指出返回lifetime与输入lifetime的关系。
如果没这个信息，main函数不知道result至少需要活多久，是和string1一样长，还是和string2一样长，还是和两个都一样长。
在这个例子中，是和两个都一样长，

简单的说，rust之所以要引入lifetime，本质上就是在跨函数调用的时候，消除空指针错误。
使得跨函数返回指针时可能的空指针错误，都能在静态分析出来。
当然，这是个may analysis，rust编译器可能有误报。

