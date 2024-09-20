use lazy_static::lazy_static;
use crate::extract_tests::{
    TestFile,
    Cursor
};

lazy_static! {
    pub static ref TEST_FILES: Vec<TestFile<'static>> = vec![
        TestFile::new(
            "no_args_from_binary_expr.rs",
            Cursor::new(2, 9),
            Cursor::new(2, 16),
        ),
        TestFile::new(
            "no_args_from_binary_expr_in_module.rs",
            Cursor::new(3, 13),
            Cursor::new(3, 20),
        ),
        TestFile::new(
            "no_args_from_binary_expr_indented.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 16),
        ),
        TestFile::new(
            "no_args_from_stmt_with_last_expr.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 10),
        ),
        TestFile::new(
            "no_args_from_stmt_unit.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 19),
        ),
        TestFile::new(
            "no_args_if.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 18),
        ),
        TestFile::new(
            "no_args_if_else.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 31),
        ),
        TestFile::new(
            "no_args_if_let_else.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 43),
        ),
        TestFile::new(
            "no_args_match.rs",
            Cursor::new(2, 5),
            Cursor::new(5, 6),
        ),
        TestFile::new(
            "no_args_while.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 21),
        ),
        TestFile::new(
            "no_args_for.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 27),
        ),
        TestFile::new(
            "no_args_from_loop_unit.rs",
            Cursor::new(2, 5),
            Cursor::new(4, 6),
        ),
        TestFile::new(
            "no_args_from_loop_with_return.rs",
            Cursor::new(2, 13),
            Cursor::new(5, 6),
        ),
        TestFile::new(
            "no_args_from_match.rs",
            Cursor::new(2, 18),
            Cursor::new(5, 6),
        ),
        TestFile::new(
            "extract_partial_block_single_line.rs",
            Cursor::new(3, 17),
            Cursor::new(3, 25),
        ),
        TestFile::new(
            "extract_partial_block.rs",
            Cursor::new(4, 19),
            Cursor::new(5, 19),
        ),
        TestFile::new(
            "argument_form_expr.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 10),
        ),
        TestFile::new(
            "argument_used_twice_form_expr.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 10),
        ),
        TestFile::new(
            "two_arguments_form_expr.rs",
            Cursor::new(4, 5),
            Cursor::new(4, 12),
        ),
        TestFile::new(
            "argument_and_locals.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 10),
        ),
        TestFile::new(
            "part_of_expr_stmt.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 8),
        ),
        TestFile::new(
            "function_expr.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 17),
        ),
        TestFile::new(
            "extract_from_nested.rs",
            Cursor::new(4, 18),
            Cursor::new(4, 25),
        ),
        TestFile::new(
            "param_from_closure.rs",
            Cursor::new(2, 27),
            Cursor::new(2, 34),
        ),
        TestFile::new(
            "extract_return_stmt.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 19),
        ),
        TestFile::new(
            "does_not_add_extra_whitespace.rs",
            Cursor::new(4, 5),
            Cursor::new(4, 19),
        ),
        TestFile::new(
            "break_stmt.rs",
            Cursor::new(3, 9),
            Cursor::new(3, 22),
        ),
        TestFile::new(
            "extract_cast.rs",
            Cursor::new(2, 13),
            Cursor::new(2, 26),
        ),
        TestFile::new(
            "method_to_freestanding.rs",
            Cursor::new(5, 9),
            Cursor::new(5, 14),
        ),
        TestFile::new(
            "method_with_reference.rs",
            Cursor::new(5, 9),
            Cursor::new(5, 24),
        ),
        TestFile::new(
            "method_with_mut.rs",
            Cursor::new(5, 9),
            Cursor::new(5, 23),
        ),
        TestFile::new(
            "variable_defined_inside_and_used_after_no_ret.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 21),
        ),
        TestFile::new(
            "variable_defined_inside_and_used_after_mutably_no_ret.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 25),
        ),
        TestFile::new(
            "two_variables_defined_inside_and_used_after_no_ret.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 19),
        ),
        TestFile::new(
            "multi_variables_defined_inside_and_used_after_mutably_no_ret.rs",
            Cursor::new(3, 5),
            Cursor::new(6, 12),
        ),
        TestFile::new(
            "nontrivial_patterns_define_variables.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 35),
        ),
        TestFile::new(
            "struct_with_two_fields_pattern_define_variables.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 56),
        ),
        TestFile::new(
            "mut_var_from_outer_scope.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 14),
        ),
        TestFile::new(
            "mut_field_from_outer_scope.rs",
            Cursor::new(4, 5),
            Cursor::new(4, 16),
        ),
        TestFile::new(
            "mut_nested_field_from_outer_scope.rs",
            Cursor::new(7, 5),
            Cursor::new(8, 24),
        ),
        TestFile::new(
            "mut_param_many_usages_stmt.rs",
            Cursor::new(11, 5),
            Cursor::new(19, 14),
        ),
        TestFile::new(
            "mut_param_many_usages_expr.rs",
            Cursor::new(11, 5),
            Cursor::new(21, 6),
        ),
        TestFile::new(
            "mut_param_because_of_mut_ref.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 13),
        ),
        TestFile::new(
            "mut_method_call.rs",
            Cursor::new(9, 5),
            Cursor::new(9, 15),
        ),
        TestFile::new(
            "copy_used_after.rs",
            Cursor::new(4, 5),
            Cursor::new(4, 17),
        ),
        TestFile::new(
            "copy_custom_used_after.rs",
            Cursor::new(6, 5),
            Cursor::new(6, 19),
        ),
        TestFile::new(
            "indented_stmts.rs",
            Cursor::new(4, 13),
            Cursor::new(5, 23),
        ),
        TestFile::new(
            "indented_stmts_inside_mod.rs",
            Cursor::new(5, 17),
            Cursor::new(6, 27),
        ),
        TestFile::new(
            "break_loop.rs",
            Cursor::new(5, 9),
            Cursor::new(7, 19),
        ),
        TestFile::new(
            "return_to_parent.rs",
            Cursor::new(4, 5),
            Cursor::new(6, 15),
        ),
        TestFile::new(
            "break_loop_with_if.rs",
            Cursor::new(5, 9),
            Cursor::new(7, 16),
        ),
        TestFile::new(
            "break_loop_nested.rs",
            Cursor::new(5, 9),
            Cursor::new(8, 10),
        ),
        TestFile::new(
            "break_loop_nested_labeled.rs",
            Cursor::new(5, 13),
            Cursor::new(5, 26),
        ),
        TestFile::new(
            "continue_loop_nested_labeled.rs",
            Cursor::new(5, 13),
            Cursor::new(5, 29),
        ),
        TestFile::new(
            "return_from_nested_loop.rs",
            Cursor::new(3, 19),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_from_nested_loop.rs",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_from_nested_and_outer_loops.rs",
            Cursor::new(4, 9),
            Cursor::new(11, 23),
        ),
        TestFile::new(
            "return_from_nested_fn.rs",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_with_value.rs",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_with_value_and_label.rs",
            Cursor::new(4, 9),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "break_with_value_and_return.rs",
            Cursor::new(3, 19),
            Cursor::new(8, 23),
        ),
        TestFile::new(
            "try_option.rs",
            Cursor::new(5, 5),
            Cursor::new(6, 19),
        ),
        TestFile::new(
            "try_option_unit.rs",
            Cursor::new(4, 5),
            Cursor::new(5, 19),
        ),
        TestFile::new(
            "try_result.rs",
            Cursor::new(4, 5),
            Cursor::new(5, 19),
        ),
        TestFile::new(
            "try_option_with_return.rs",
            Cursor::new(4, 5),
            Cursor::new(8, 19),
        ),
        TestFile::new(
            "try_result_with_return.rs",
            Cursor::new(4, 5),
            Cursor::new(8, 19),
        ),
        TestFile::new(
            "try_and_return_ok.rs",
            Cursor::new(4, 5),
            Cursor::new(8, 19),
        ),
        TestFile::new(
            "param_usage_in_macro.rs",
            Cursor::new(7, 5),
            Cursor::new(7, 25),
        ),
        TestFile::new(
            "param_usage_in_macro_with_nested_tt.rs",
            Cursor::new(8, 5),
            Cursor::new(8, 35),
        ),
        TestFile::new(
            "param_usage_in_macro_with_nested_tt_2.rs",
            Cursor::new(9, 9),
            Cursor::new(9, 44),
        ),
        TestFile::new(
            "extract_with_await.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 29),
        ),
        TestFile::new(
            "extract_with_await_and_result_not_producing_match_expr.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 13),
        ),
        TestFile::new(
            "extract_with_await_and_result_producing_match_expr.rs",
            Cursor::new(4, 19),
            Cursor::new(9, 23),
        ),
        TestFile::new(
            "extract_with_await_in_args.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 49),
        ),
        TestFile::new(
            "extract_does_not_tear_comments_apart.rs",
            Cursor::new(2, 7),
            Cursor::new(5, 7),
        ),
        TestFile::new(
            "extract_does_not_tear_body_apart.rs",
            Cursor::new(2, 5),
            Cursor::new(3, 2),
        ),
        TestFile::new(
            "extract_does_not_wrap_res_in_res.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 11),
        ),
        TestFile::new(
            "extract_mut_ref_param_has_no_mut_binding_in_loop.rs",
            Cursor::new(9, 9),
            Cursor::new(9, 19),
        ),
        TestFile::new(
            "extract_with_macro_arg.rs",
            Cursor::new(6, 5),
            Cursor::new(6, 15),
        ),
        TestFile::new(
            "unresolveable_types_default_to_placeholder.rs",
            Cursor::new(3, 13),
            Cursor::new(3, 18),
        ),
        TestFile::new(
            "reference_mutable_param_with_further_usages.rs",
            Cursor::new(6, 5),
            Cursor::new(6, 21),
        ),
        TestFile::new(
            "reference_mutable_param_without_further_usages.rs",
            Cursor::new(6, 5),
            Cursor::new(6, 21),
        ),
        TestFile::new(
            "does_not_import_control_flow.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 34),
        ),
        TestFile::new(
            "extract_function_copies_comment_at_start.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 15),
        ),
        TestFile::new(
            "extract_function_copies_comment_in_between.rs",
            Cursor::new(2, 15),
            Cursor::new(5, 15),
        ),
        TestFile::new(
            "extract_function_copies_comment_at_end.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 21),
        ),
        TestFile::new(
            "extract_function_copies_comment_indented.rs",
            Cursor::new(3, 5),
            Cursor::new(6, 6),
        ),
        TestFile::new(
            "extract_function_does_preserve_whitespace.rs",
            Cursor::new(3, 5),
            Cursor::new(5, 15),
        ),
        TestFile::new(
            "extract_function_long_form_comment.rs",
            Cursor::new(3, 5),
            Cursor::new(4, 15),
        ),
        TestFile::new(
            "it_should_not_generate_duplicate_function_names.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 17),
        ),
        TestFile::new(
            "should_increment_suffix_until_it_finds_space.rs",
            Cursor::new(6, 5),
            Cursor::new(6, 17),
        ),
        TestFile::new(
            "extract_method_from_trait_impl.rs",
            Cursor::new(8, 9),
            Cursor::new(8, 21),
        ),
        TestFile::new(
            "extract_method_from_trait_with_existing_non_empty_impl_block.rs",
            Cursor::new(12, 9),
            Cursor::new(12, 21),
        ),
        TestFile::new(
            "extract_function_from_trait_with_existing_non_empty_impl_block.rs",
            Cursor::new(12, 29),
            Cursor::new(12, 36),
        ),
        TestFile::new(
            "extract_method_from_trait_with_multiple_existing_impl_blocks.rs",
            Cursor::new(22, 9),
            Cursor::new(22, 21),
        ),
        TestFile::new(
            "extract_method_from_trait_with_multiple_existing_trait_impl_blocks.rs",
            Cursor::new(30, 9),
            Cursor::new(30, 21),
        ),
        TestFile::new(
            "closure_arguments.rs",
            Cursor::new(4, 5),
            Cursor::new(4, 38),
        ),
        TestFile::new(
            "preserve_generics.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 14),
        ),
        TestFile::new(
            "dont_emit_type_with_hidden_lifetime_parameter.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 14),
        ),
        TestFile::new(
            "preserve_generics_from_body.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 19),
        ),
        TestFile::new(
            "filter_unused_generics.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 14),
        ),
        TestFile::new(
            "empty_generic_param_list.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 14),
        ),
        TestFile::new(
            "preserve_where_clause.rs",
            Cursor::new(2, 5),
            Cursor::new(2, 14),
        ),
        TestFile::new(
            "filter_unused_where_clause.rs",
            Cursor::new(3, 5),
            Cursor::new(3, 14),
        ),
        TestFile::new(
            "nested_generics.rs",
            Cursor::new(5, 9),
            Cursor::new(5, 30),
        ),
        TestFile::new(
            "filters_unused_nested_generics.rs",
            Cursor::new(5, 9),
            Cursor::new(5, 30),
        ),
        TestFile::new(
            "nested_where_clauses.rs",
            Cursor::new(5, 9),
            Cursor::new(5, 30),
        ),
        TestFile::new(
            "filters_unused_nested_where_clauses.rs",
            Cursor::new(5, 9),
            Cursor::new(5, 30),
        ),
        TestFile::new(
            "tail_expr_no_extra_control_flow.rs",
            Cursor::new(3, 5),
            Cursor::new(6, 11),
        ),
        TestFile::new(
            "non_tail_expr_of_tail_expr_loop.rs",
            Cursor::new(3, 9),
            Cursor::new(5, 10),
        ),
        TestFile::new(
            "non_tail_expr_of_tail_if_block.rs",
            Cursor::new(4, 17),
            Cursor::new(8, 10),
        ),
        TestFile::new(
            "tail_expr_of_tail_block_nested.rs",
            Cursor::new(4, 9),
            Cursor::new(11, 10),
        ),
        TestFile::new(
            "non_tail_expr_with_comment_of_tail_expr_loop.rs",
            Cursor::new(3, 9),
            Cursor::new(6, 10),
        ),
        TestFile::new(
            "comments_in_block_expr.rs",
            Cursor::new(2, 13),
            Cursor::new(8, 6),
        ),
    ];
}
