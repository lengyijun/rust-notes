use std::collections::HashSet;

#[derive(Hash,Debug)]
struct S(usize);

impl PartialEq for &S {
    fn eq(&self, other: &Self) -> bool {
        // self.0== other.0
        *self as *const S as usize == *other as *const S as usize
    }
}
impl Eq for &S{}

fn main() {
    let mut h=HashSet::<&S>::new();

    let a=S(0);
    let b=S(0);
    h.insert(&a);
    h.insert(&b);

    dbg!(h);

}

// other link:
// https://stackoverflow.com/questions/33847537/how-do-i-make-a-pointer-hashable
//
// Conclusion:
// 1. HashMap<&T,()> 并不代表他的key是一个指针，而是取决于&T 有没有实现Hash 和 Eq
// 如果 &T 实现了 Hash和Eq ,那么key确实是指针
// 如果 &T 没有实现 Hash 和 Eq, 那么会自动解引用，尝试去找 T的Hash和Eq
// 这里不能看类型声明想当然，要自己figure out. 个人觉得这里的自动解引用挺令人误解的
//
// Hash(Box<T>) = Hash(T)  // 对值Hash. 如果没有对 Box<T> 实现自己的Hash
// Hash(&T) = Hash(T)      // 对值Hash. 如果没有对 &T 实现自己的Hash
// Hash(* const T) = Hash(* const T as usize)   // 对地址Hash
//
