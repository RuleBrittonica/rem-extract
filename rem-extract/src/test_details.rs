use lazy_static::lazy_static;
use crate::extract_tests::TestFile;
use crate::extraction::Cursor;
lazy_static! {
    pub static ref TEST_FILES: Vec<TestFile<'static>> = vec![
        TestFile::new(
            "no_args_from_binary_expr",
            Cursor::new(2, 9),
            Cursor::new(2, 14),
        ),
        TestFile::new(
            "no_args_from_binary_expr_in_module",
            Cursor::new(3, 13),
            Cursor::new(3, 18),
        ),
        TestFile::new(
            "no_args_from_binary_expr_indented",
            Cursor::new(2, 5),
            Cursor::new(2, 14),
        ),
        TestFile::new(
            "no_args_from_stmt_with_last_expr",
            Cursor::new(3, 5),
            Cursor::new(4, 10),
        ),
        TestFile::new(
            "no_args_from_stmt_unit",
            Cursor::new(3, 5),
            Cursor::new(4, 19),
        ),
        TestFile::new(
            "no_args_if",
            Cursor::new(2, 5),
            Cursor::new(2, 16),
        ),
        TestFile::new(
            "no_args_if_else",
            Cursor::new(2, 5),
            Cursor::new(2, 29),
        ),
        TestFile::new(
            "no_args_if_let_else",
            Cursor::new(2, 5),
            Cursor::new(2, 41),
        ),
        TestFile::new(
            "no_args_match",
            Cursor::new(2, 5),
            Cursor::new(5, 6),
        ),
        TestFile::new(
            "no_args_while",
            Cursor::new(2, 5),
            Cursor::new(2, 19),
        ),
        TestFile::new(
            "no_args_for",
            Cursor::new(2, 5),
            Cursor::new(2, 25),
        ),
        TestFile::new(
            "no_args_from_loop_unit",
            Cursor::new(2, 5),
            Cursor::new(4, 6),
        ),
        TestFile::new(
            "no_args_from_loop_with_return",
            Cursor::new(2, 13),
            Cursor::new(5, 6),
        ),
        TestFile::new(
            "no_args_from_match",
            Cursor::new(2, 18),
            Cursor::new(5, 6),
        ),
        TestFile::new(
            "extract_partial_block_single_line",
            Cursor::new(3, 17),
            Cursor::new(3, 23),
        ),
        TestFile::new(
            "extract_partial_block",
            Cursor::new(4, 19),
            Cursor::new(5, 19),
        ),
        TestFile::new(
            "argument_form_expr",
            Cursor::new(3, 5),
            Cursor::new(3, 8),
        ),
        TestFile::new(
            "argument_used_twice_form_expr",
            Cursor::new(3, 5),
            Cursor::new(3, 8),
        ),
        TestFile::new(
            "two_arguments_form_expr",
            Cursor::new(4, 5),
            Cursor::new(4, 10),
        ),
        TestFile::new(
            "argument_and_locals",
            Cursor::new(3, 5),
            Cursor::new(4, 10),
        ),
        TestFile::new(
            "part_of_expr_stmt",
            Cursor::new(2, 5),
            Cursor::new(2, 6),
        ),
        TestFile::new(
            "function_expr",
            Cursor::new(2, 5),
            Cursor::new(2, 15),
        ),
        TestFile::new(
            "extract_from_nested",
            Cursor::new(4, 18),
            Cursor::new(4, 23),
        ),
        TestFile::new(
            "param_from_closure",
            Cursor::new(2, 27),
            Cursor::new(2, 32),
        ),
        TestFile::new(
            "extract_return_stmt",
            Cursor::new(2, 5),
            Cursor::new(2, 17),
        ),
        TestFile::new(
            "does_not_add_extra_whitespace",
            Cursor::new(4, 5),
            Cursor::new(4, 17),
        ),
        TestFile::new(
            "break_stmt",
            Cursor::new(3, 9),
            Cursor::new(3, 20),
        ),
        TestFile::new(
            "extract_cast",
            Cursor::new(2, 13),
            Cursor::new(2, 24),
        ),
        TestFile::new(
            "method_to_freestanding",
            Cursor::new(5, 9),
            Cursor::new(5, 12),
        ),
        TestFile::new(
            "method_with_reference",
            Cursor::new(5, 9),
            Cursor::new(5, 22),
        ),
        TestFile::new(
            "method_with_mut",
            Cursor::new(5, 9),
            Cursor::new(5, 21),
        ),
        TestFile::new(
            "variable_defined_inside_and_used_after_no_ret",
            Cursor::new(3, 5),
            Cursor::new(3, 19),
        ),
        TestFile::new(
            "variable_defined_inside_and_used_after_mutably_no_ret",
            Cursor::new(3, 5),
            Cursor::new(3, 23),
        ),
        TestFile::new(
            "two_variables_defined_inside_and_used_after_no_ret",
            Cursor::new(3, 5),
            Cursor::new(4, 19),
        ),
        TestFile::new(
            "multi_variables_defined_inside_and_used_after_mutably_no_ret",
            Cursor::new(3, 5),
            Cursor::new(6, 12),
        ),
        TestFile::new(
            "nontrivial_patterns_define_variables",
            Cursor::new(3, 5),
            Cursor::new(3, 33),
        ),
        TestFile::new(
            "struct_with_two_fields_pattern_define_variables",
            Cursor::new(3, 5),
            Cursor::new(3, 54),
        ),
        TestFile::new(
            "mut_var_from_outer_scope",
            Cursor::new(3, 5),
            Cursor::new(3, 12),
        ),
        TestFile::new(
            "mut_field_from_outer_scope",
            Cursor::new(4, 5),
            Cursor::new(4, 14),
        ),
        TestFile::new(
            "mut_nested_field_from_outer_scope",
            Cursor::new(7, 5),
            Cursor::new(8, 24),
        ),
        TestFile::new(
            "mut_param_many_usages_stmt",
            Cursor::new(11, 5),
            Cursor::new(19, 14),
        ),
        TestFile::new(
            "mut_param_many_usages_expr",
            Cursor::new(11, 5),
            Cursor::new(21, 6),
        ),
        TestFile::new(
            "mut_param_because_of_mut_ref",
            Cursor::new(3, 5),
            Cursor::new(4, 13),
        ),
        TestFile::new(
            "mut_method_call",
            Cursor::new(9, 5),
            Cursor::new(9, 13),
        ),
        TestFile::new(
            "copy_used_after",
            Cursor::new(4, 5),
            Cursor::new(4, 15),
        ),
        TestFile::new(
            "copy_custom_used_after",
            Cursor::new(6, 5),
            Cursor::new(6, 17),
        ),
        TestFile::new(
            "indented_stmts",
            Cursor::new(4, 13),
            Cursor::new(5, 23),
        ),
        TestFile::new(
            "indented_stmts_inside_mod",
            Cursor::new(5, 17),
            Cursor::new(6, 27),
        ),
        TestFile::new(
            "break_loop",
            Cursor::new(5, 9),
            Cursor::new(7, 19),
        ),
        TestFile::new(
            "return_to_parent",
            Cursor::new(4, 5),
            Cursor::new(6, 15),
        ),
        TestFile::new(
            "break_loop_with_if",
            Cursor::new(5, 9),
            Cursor::new(7, 16),
        ),
        TestFile::new(
            "break_loop_nested",
            Cursor::new(5, 9),
            Cursor::new(8, 10),
        ),
        TestFile::new(
            "break_loop_nested_labeled",
            Cursor::new(5, 13),
            Cursor::new(5, 24),
        ),
        TestFile::new(
            "continue_loop_nested_labeled",
            Cursor::new(5, 13),
            Cursor::new(5, 27),
        ),
        TestFile::new(
            "return_from_nested_loop",
            Cursor::new(3, 19),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_from_nested_loop",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_from_nested_and_outer_loops",
            Cursor::new(4, 9),
            Cursor::new(11, 23),
        ),
        TestFile::new(
            "return_from_nested_fn",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_with_value",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_with_value_and_label",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_with_value_and_return",
            Cursor::new(3, 19),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "try_option",
            Cursor::new(5, 5),
            Cursor::new(6, 19),
        ),
        TestFile::new(
            "try_option_unit",
            Cursor::new(4, 5),
            Cursor::new(5, 19),
        ),
        TestFile::new(
            "try_result",
            Cursor::new(4, 5),
            Cursor::new(5, 19),
        ),
        TestFile::new(
            "try_option_with_return",
            Cursor::new(4, 5),
            Cursor::new(8, 19),
        ),
        TestFile::new(
            "try_result_with_return",
            Cursor::new(4, 5),
            Cursor::new(8, 19),
        ),
        TestFile::new(
            "try_and_return_ok",
            Cursor::new(4, 5),
            Cursor::new(8, 19),
        ),
        TestFile::new(
            "param_usage_in_macro",
            Cursor::new(7, 5),
            Cursor::new(7, 23),
        ),
        TestFile::new(
            "param_usage_in_macro_with_nested_tt",
            Cursor::new(8, 5),
            Cursor::new(8, 33),
        ),
        TestFile::new(
            "param_usage_in_macro_with_nested_tt_2",
            Cursor::new(9, 9),
            Cursor::new(9, 42),
        ),
        TestFile::new(
            "extract_with_await",
            Cursor::new(3, 5),
            Cursor::new(3, 27),
        ),
        TestFile::new(
            "extract_with_await_and_result_not_producing_match_expr",
            Cursor::new(3, 5),
            Cursor::new(4, 13),
        ),
        TestFile::new(
            "extract_with_await_and_result_producing_match_expr",
            Cursor::new(4, 19),
            Cursor::new(9, 23),
        ),
        TestFile::new(
            "extract_with_await_in_args",
            Cursor::new(3, 5),
            Cursor::new(3, 47),
        ),
        TestFile::new(
            "extract_does_not_tear_comments_apart",
            Cursor::new(2, 7),
            Cursor::new(5, 7),
        ),
        TestFile::new(
            "extract_does_not_tear_body_apart",
            Cursor::new(2, 5),
            Cursor::new(3, 2),
        ),
        TestFile::new(
            "extract_does_not_wrap_res_in_res",
            Cursor::new(3, 5),
            Cursor::new(4, 11),
        ),
        TestFile::new(
            "extract_mut_ref_param_has_no_mut_binding_in_loop",
            Cursor::new(9, 9),
            Cursor::new(9, 17),
        ),
        TestFile::new(
            "extract_with_macro_arg",
            Cursor::new(6, 5),
            Cursor::new(6, 13),
        ),
        TestFile::new(
            "unresolveable_types_default_to_placeholder",
            Cursor::new(3, 13),
            Cursor::new(3, 16),
        ),
        TestFile::new(
            "reference_mutable_param_with_further_usages",
            Cursor::new(6, 5),
            Cursor::new(6, 19),
        ),
        TestFile::new(
            "reference_mutable_param_without_further_usages",
            Cursor::new(6, 5),
            Cursor::new(6, 19),
        ),
        TestFile::new(
            "does_not_import_control_flow",
            Cursor::new(3, 5),
            Cursor::new(3, 32),
        ),
        TestFile::new(
            "extract_function_copies_comment_at_start",
            Cursor::new(3, 5),
            Cursor::new(4, 15),
        ),
        TestFile::new(
            "extract_function_copies_comment_in_between",
            Cursor::new(2, 15),
            Cursor::new(5, 15),
        ),
        TestFile::new(
            "extract_function_copies_comment_at_end",
            Cursor::new(3, 5),
            Cursor::new(4, 21),
        ),
        TestFile::new(
            "extract_function_copies_comment_indented",
            Cursor::new(3, 5),
            Cursor::new(6, 6),
        ),
        TestFile::new(
            "extract_function_does_preserve_whitespace",
            Cursor::new(3, 5),
            Cursor::new(5, 15),
        ),
        TestFile::new(
            "extract_function_long_form_comment",
            Cursor::new(3, 5),
            Cursor::new(4, 15),
        ),
        TestFile::new(
            "it_should_not_generate_duplicate_function_names",
            Cursor::new(2, 5),
            Cursor::new(2, 15),
        ),
        TestFile::new(
            "should_increment_suffix_until_it_finds_space",
            Cursor::new(6, 5),
            Cursor::new(6, 15),
        ),
        TestFile::new(
            "extract_method_from_trait_impl",
            Cursor::new(8, 9),
            Cursor::new(8, 19),
        ),
        TestFile::new(
            "extract_method_from_trait_with_existing_non_empty_impl_block",
            Cursor::new(12, 9),
            Cursor::new(12, 19),
        ),
        TestFile::new(
            "extract_function_from_trait_with_existing_non_empty_impl_block",
            Cursor::new(12, 29),
            Cursor::new(12, 34),
        ),
        TestFile::new(
            "extract_method_from_trait_with_multiple_existing_impl_blocks",
            Cursor::new(22, 9),
            Cursor::new(22, 19),
        ),
        TestFile::new(
            "extract_method_from_trait_with_multiple_existing_trait_impl_blocks",
            Cursor::new(30, 9),
            Cursor::new(30, 19),
        ),
        TestFile::new(
            "closure_arguments",
            Cursor::new(4, 5),
            Cursor::new(4, 36),
        ),
        TestFile::new(
            "preserve_generics",
            Cursor::new(2, 5),
            Cursor::new(2, 12),
        ),
        TestFile::new(
            "dont_emit_type_with_hidden_lifetime_parameter",
            Cursor::new(3, 5),
            Cursor::new(3, 12),
        ),
        TestFile::new(
            "preserve_generics_from_body",
            Cursor::new(2, 5),
            Cursor::new(2, 17),
        ),
        TestFile::new(
            "filter_unused_generics",
            Cursor::new(3, 5),
            Cursor::new(3, 12),
        ),
        TestFile::new(
            "empty_generic_param_list",
            Cursor::new(3, 5),
            Cursor::new(3, 12),
        ),
        TestFile::new(
            "preserve_where_clause",
            Cursor::new(2, 5),
            Cursor::new(2, 12),
        ),
        TestFile::new(
            "filter_unused_where_clause",
            Cursor::new(3, 5),
            Cursor::new(3, 12),
        ),
        TestFile::new(
            "nested_generics",
            Cursor::new(5, 9),
            Cursor::new(5, 28),
        ),
        TestFile::new(
            "filters_unused_nested_generics",
            Cursor::new(5, 9),
            Cursor::new(5, 28),
        ),
        TestFile::new(
            "nested_where_clauses",
            Cursor::new(5, 9),
            Cursor::new(5, 28),
        ),
        TestFile::new(
            "filters_unused_nested_where_clauses",
            Cursor::new(5, 9),
            Cursor::new(5, 28),
        ),
        TestFile::new(
            "tail_expr_no_extra_control_flow",
            Cursor::new(3, 5),
            Cursor::new(6, 11),
        ),
        TestFile::new(
            "non_tail_expr_of_tail_expr_loop",
            Cursor::new(3, 9),
            Cursor::new(5, 10),
        ),
        TestFile::new(
            "non_tail_expr_of_tail_if_block",
            Cursor::new(4, 17),
            Cursor::new(8, 10),
        ),
        TestFile::new(
            "tail_expr_of_tail_block_nested",
            Cursor::new(4, 9),
            Cursor::new(11, 10),
        ),
        TestFile::new(
            "non_tail_expr_with_comment_of_tail_expr_loop",
            Cursor::new(3, 9),
            Cursor::new(6, 10),
        ),
        TestFile::new(
            "comments_in_block_expr",
            Cursor::new(2, 13),
            Cursor::new(8, 6),
        ),
    ];
}
