loan不是孤立的存在的。一个Loan 一生出来就有一个Origin包住他了
不会有 (Loan,Loan) 这种规则


# (point,point)
`output::cfg_edge(point1, point2)`

# (path,path)
`output::child_path(child, parent)`
`initialization::ancestor_path(Parent, Child)`

# (path,point)
`output::path_accessed_at_base(path, point)`
`output::path_assigned_at_base(path, point)`
`output::path_moved_at_base(path, point)`
`initialization::path_accessed_at(path, point)`
`initialization::path_assigned_at(path, point)`
`initialization::path_moved_at(path, point)`
`initialization::path_maybe_initialized_on_exit(path, point)`
`initialization::path_maybe_uninitialized_on_exit(path, point)`
`initialization::move_error(Path, point)`

# (path,var)
`output::path_is_var(path, var)`
`initialization::path_begins_with_var(path, var)`

# (var,origin)
`output::drop_of_var_derefs_origin(var, origin)`
`output::use_of_var_derefs_origin(variable, origin)`

# (var,point)
`output::var_defined_at(var, point)`
`output::var_dropped_at(var, point)`
`output::var_used_at(var, point)`
`initialization::var_maybe_partly_initialized_on_exit`

# (point, loan)
`output::loan_invalidated_at(point, loan)`
`output::loan_killed_at(loan, point)`

# (origin, loan, point)
`output::loan_issued_at(origin, loan, point)`

# (origin,loan)
`output::placeholder(origin, loan)`

# (origin,origin,point)
`output::subset_base(origin1, origin2, point)`

# (origin,origin)
`output::known_subset(origin,origin)`

