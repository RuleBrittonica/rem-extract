use lazy_static::lazy_static;
use crate::test::TestFile;

lazy_static! {
    pub static ref TEST_FILES: Vec<TestFile<'static>> = vec![
        TestFile {
            input_file: "no_args_from_binary_expr.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "no_args_from_binary_expr_in_module.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "no_args_from_binary_expr_indented.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "no_args_from_stmt_with_last_expr.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "no_args_from_stmt_unit.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "no_args_if.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "no_args_if_else.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "no_args_if_let_else.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "no_args_match.rs",
            start_line: 2,
            end_line: 5,
        },
        TestFile {
            input_file: "no_args_while.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "no_args_for.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "no_args_from_loop_unit.rs",
            start_line: 2,
            end_line: 4,
        },
        TestFile {
            input_file: "no_args_from_loop_with_return.rs",
            start_line: 2,
            end_line: 5,
        },
        TestFile {
            input_file: "no_args_from_match.rs",
            start_line: 2,
            end_line: 5,
        },
        TestFile {
            input_file: "extract_partial_block_single_line.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "extract_partial_block.rs",
            start_line: 4,
            end_line: 5,
        },
        TestFile {
            input_file: "argument_form_expr.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "argument_used_twice_form_expr.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "two_arguments_form_expr.rs",
            start_line: 4,
            end_line: 4,
        },
        TestFile {
            input_file: "argument_and_locals.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "part_of_expr_stmt.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "function_expr.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "extract_from_nested.rs",
            start_line: 4,
            end_line: 4,
        },
        TestFile {
            input_file: "param_from_closure.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "extract_return_stmt.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "does_not_add_extra_whitespace.rs",
            start_line: 4,
            end_line: 4,
        },
        TestFile {
            input_file: "break_stmt.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "extract_cast.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "method_to_freestanding.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "method_with_reference.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "method_with_mut.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "variable_defined_inside_and_used_after_no_ret.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "variable_defined_inside_and_used_after_mutably_no_ret.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "two_variables_defined_inside_and_used_after_no_ret.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "multi_variables_defined_inside_and_used_after_mutably_no_ret.rs",
            start_line: 3,
            end_line: 6,
        },
        TestFile {
            input_file: "nontrivial_patterns_define_variables.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "struct_with_two_fields_pattern_define_variables.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "mut_var_from_outer_scope.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "mut_field_from_outer_scope.rs",
            start_line: 4,
            end_line: 4,
        },
        TestFile {
            input_file: "mut_nested_field_from_outer_scope.rs",
            start_line: 7,
            end_line: 8,
        },
        TestFile {
            input_file: "mut_param_many_usages_stmt.rs",
            start_line: 11,
            end_line: 19,
        },
        TestFile {
            input_file: "mut_param_many_usages_expr.rs",
            start_line: 11,
            end_line: 21,
        },
        TestFile {
            input_file: "mut_param_because_of_mut_ref.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "mut_method_call.rs",
            start_line: 9,
            end_line: 9,
        },
        TestFile {
            input_file: "copy_used_after.rs",
            start_line: 4,
            end_line: 4,
        },
        TestFile {
            input_file: "copy_custom_used_after.rs",
            start_line: 6,
            end_line: 6,
        },
        TestFile {
            input_file: "indented_stmts.rs",
            start_line: 4,
            end_line: 5,
        },
        TestFile {
            input_file: "indented_stmts_inside_mod.rs",
            start_line: 5,
            end_line: 6,
        },
        TestFile {
            input_file: "break_loop.rs",
            start_line: 5,
            end_line: 7,
        },
        TestFile {
            input_file: "return_to_parent.rs",
            start_line: 4,
            end_line: 6,
        },
        TestFile {
            input_file: "break_loop_with_if.rs",
            start_line: 5,
            end_line: 7,
        },
        TestFile {
            input_file: "break_loop_nested.rs",
            start_line: 5,
            end_line: 8,
        },
        TestFile {
            input_file: "break_loop_nested_labeled.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "continue_loop_nested_labeled.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "return_from_nested_loop.rs",
            start_line: 3,
            end_line: 8,
        },
        TestFile {
            input_file: "break_from_nested_loop.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "break_from_nested_and_outer_loops.rs",
            start_line: 4,
            end_line: 11,
        },
        TestFile {
            input_file: "return_from_nested_fn.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "break_with_value.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "break_with_value_and_label.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "break_with_value_and_return.rs",
            start_line: 3,
            end_line: 8,
        },
        TestFile {
            input_file: "try_option.rs",
            start_line: 5,
            end_line: 6,
        },
        TestFile {
            input_file: "try_option_unit.rs",
            start_line: 4,
            end_line: 5,
        },
        TestFile {
            input_file: "try_result.rs",
            start_line: 4,
            end_line: 5,
        },
        TestFile {
            input_file: "try_option_with_return.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "try_result_with_return.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "try_and_return_ok.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "param_usage_in_macro.rs",
            start_line: 7,
            end_line: 7,
        },
        TestFile {
            input_file: "param_usage_in_macro_with_nested_tt.rs",
            start_line: 8,
            end_line: 8,
        },
        TestFile {
            input_file: "param_usage_in_macro_with_nested_tt_2.rs",
            start_line: 9,
            end_line: 9,
        },
        TestFile {
            input_file: "extract_with_await.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "extract_with_await_and_result_not_producing_match_expr.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "extract_with_await_and_result_producing_match_expr.rs",
            start_line: 4,
            end_line: 9,
        },
        TestFile {
            input_file: "extract_with_await_in_args.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "extract_does_not_tear_comments_apart.rs",
            start_line: 2,
            end_line: 5,
        },
        TestFile {
            input_file: "extract_does_not_tear_body_apart.rs",
            start_line: 2,
            end_line: 3,
        },
        TestFile {
            input_file: "extract_does_not_wrap_res_in_res.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "extract_mut_ref_param_has_no_mut_binding_in_loop.rs",
            start_line: 9,
            end_line: 9,
        },
        TestFile {
            input_file: "extract_with_macro_arg.rs",
            start_line: 6,
            end_line: 6,
        },
        TestFile {
            input_file: "unresolveable_types_default_to_placeholder.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "reference_mutable_param_with_further_usages.rs",
            start_line: 6,
            end_line: 6,
        },
        TestFile {
            input_file: "reference_mutable_param_without_further_usages.rs",
            start_line: 6,
            end_line: 6,
        },
        TestFile {
            input_file: "does_not_import_control_flow.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "extract_function_copies_comment_at_start.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "extract_function_copies_comment_in_between.rs",
            start_line: 2,
            end_line: 5,
        },
        TestFile {
            input_file: "extract_function_copies_comment_at_end.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "extract_function_copies_comment_indented.rs",
            start_line: 3,
            end_line: 6,
        },
        TestFile {
            input_file: "extract_function_does_preserve_whitespace.rs",
            start_line: 3,
            end_line: 5,
        },
        TestFile {
            input_file: "extract_function_long_form_comment.rs",
            start_line: 3,
            end_line: 4,
        },
        TestFile {
            input_file: "it_should_not_generate_duplicate_function_names.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "should_increment_suffix_until_it_finds_space.rs",
            start_line: 6,
            end_line: 6,
        },
        TestFile {
            input_file: "extract_method_from_trait_impl.rs",
            start_line: 8,
            end_line: 8,
        },
        TestFile {
            input_file: "extract_method_from_trait_with_existing_non_empty_impl_block.rs",
            start_line: 12,
            end_line: 12,
        },
        TestFile {
            input_file: "extract_function_from_trait_with_existing_non_empty_impl_block.rs",
            start_line: 12,
            end_line: 12,
        },
        TestFile {
            input_file: "extract_method_from_trait_with_multiple_existing_impl_blocks.rs",
            start_line: 22,
            end_line: 22,
        },
        TestFile {
            input_file: "extract_method_from_trait_with_multiple_existing_trait_impl_blocks.rs",
            start_line: 30,
            end_line: 30,
        },
        TestFile {
            input_file: "closure_arguments.rs",
            start_line: 4,
            end_line: 4,
        },
        TestFile {
            input_file: "preserve_generics.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "dont_emit_type_with_hidden_lifetime_parameter.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "preserve_generics_from_body.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "filter_unused_generics.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "empty_generic_param_list.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "preserve_where_clause.rs",
            start_line: 2,
            end_line: 2,
        },
        TestFile {
            input_file: "filter_unused_where_clause.rs",
            start_line: 3,
            end_line: 3,
        },
        TestFile {
            input_file: "nested_generics.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "filters_unused_nested_generics.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "nested_where_clauses.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "filters_unused_nested_where_clauses.rs",
            start_line: 5,
            end_line: 5,
        },
        TestFile {
            input_file: "tail_expr_no_extra_control_flow.rs",
            start_line: 3,
            end_line: 6,
        },
        TestFile {
            input_file: "non_tail_expr_of_tail_expr_loop.rs",
            start_line: 3,
            end_line: 5,
        },
        TestFile {
            input_file: "non_tail_expr_of_tail_if_block.rs",
            start_line: 4,
            end_line: 8,
        },
        TestFile {
            input_file: "tail_expr_of_tail_block_nested.rs",
            start_line: 4,
            end_line: 11,
        },
        TestFile {
            input_file: "non_tail_expr_with_comment_of_tail_expr_loop.rs",
            start_line: 3,
            end_line: 6,
        },
        TestFile {
            input_file: "comments_in_block_expr.rs",
            start_line: 2,
            end_line: 8,
        },
    ];
}
