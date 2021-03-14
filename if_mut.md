```
use std::collections::HashMap;

trait A {
    fn get(&mut self, name: &str) -> &mut Vec<i32>;
}

struct B(HashMap<String, Vec<i32>>);

impl A for B {
   fn get(&mut self, name: &str) -> &mut Vec<i32> {
        if self.0.contains_key(name) {
            return self.0.get_mut(name).unwrap();
        }
        self.0.entry(name.to_string()).or_default()
    }
}
```

在if中，不能有可变引用
