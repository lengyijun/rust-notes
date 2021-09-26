
Type naive_subset origin -> origin -> prop.
Type naive_origin_contain_loan_on_entry point -> origin -> loan -> prop.
Type naive_error point -> loan ->  prop.

Type datafrog_opt_subset origin -> origin -> prop.
Type datafrop_opt_origin_contain_loan_on_entry point -> origin -> loan -> prop.
Type datafrog_opt_error point -> loan -> prop.

Type dead_borrow_region_can_reach_root point -> origin -> loan -> prop.
Type dead_borrow_region_can_reach_dead point -> origin -> loan -> prop.
Type dying_region_requires -> origin -> point -> point -> loan -> prop.
Type live_to_dying_regions -> origin ->  origin -> point -> point2 -> prop.
Type dying_can_reach_origins -> origin -> point -> point -> prop.
Type dying_can_reach -> origin -> origin -> point -> point2 -> prop.
Type dying_can_reach_live -> origin -> origin -> point -> point -> prop.
