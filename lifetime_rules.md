1. 引用的lifetime,不能超过这个值的作用域
2. 可变引用的lifetime,不可以与其他引用的lifetime重叠
3. 共享引用的lifetime内，拥有者暂时不可写转移，但是可以读
4. 可变引用的lifetime内，拥有者暂时不可读写转移
5. 可变引用如果从拥有者生成，拥有者必须可变
