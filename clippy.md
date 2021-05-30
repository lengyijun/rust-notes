看了一遍clippy，发现有些是挺容易犯的错误，有些错误背后则有很深刻的认识：

mem_forget
option_option
path_buf_push_overwrite
pattern_type_mismatch

panic unsafe 一定要文档

// 看不懂
// &Option<&T>
ref_option_ref

# 源码值得一看的
needless_continue
needless_for_each
needless_pass_by_value
redundant_else (比较简单，可以优先看一下)

