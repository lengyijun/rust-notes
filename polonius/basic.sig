Kind origin type.
Kind loan type.
Kind point type.

Type origin_live_on_entry point -> origin -> prop.
Type loan_issued_at point -> origin -> loan -> prop.
Type cfg_edge point -> point -> prop.
Type loan_invalidated_at point -> loan -> prop.
Type loan_killed_at point -> loan -> prop.
Type subset_base origin -> origin -> prop.

