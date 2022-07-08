```
// https://huawei-events.de/en/gsts22-j83dco-vod.htm
#include <cstdio>
#include<vector>

int main(){
  std::vector<int> v;
  v.push_back(42);
  v.push_back(43);

  const auto it = v.cbegin();
  v.push_back(44);
  printf("%d",*it);
}
```

