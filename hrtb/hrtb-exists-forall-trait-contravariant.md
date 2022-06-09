impl<'a> Trait<fn(&'a u32)> for () {}

impl Trait<for<'a> fn(&'a u32)> for () {}


注意以上的区别

