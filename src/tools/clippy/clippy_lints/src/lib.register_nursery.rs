// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_group(true, "clippy::nursery", Some("clippy_nursery"), vec![
    LintId::of(attrs::EMPTY_LINE_AFTER_OUTER_ATTR),
    LintId::of(cognitive_complexity::COGNITIVE_COMPLEXITY),
    LintId::of(copies::BRANCHES_SHARING_CODE),
    LintId::of(equatable_if_let::EQUATABLE_IF_LET),
    LintId::of(fallible_impl_from::FALLIBLE_IMPL_FROM),
    LintId::of(floating_point_arithmetic::IMPRECISE_FLOPS),
    LintId::of(floating_point_arithmetic::SUBOPTIMAL_FLOPS),
    LintId::of(future_not_send::FUTURE_NOT_SEND),
    LintId::of(index_refutable_slice::INDEX_REFUTABLE_SLICE),
    LintId::of(let_if_seq::USELESS_LET_IF_SEQ),
    LintId::of(missing_const_for_fn::MISSING_CONST_FOR_FN),
    LintId::of(mutable_debug_assertion::DEBUG_ASSERT_WITH_MUT_CALL),
    LintId::of(mutex_atomic::MUTEX_ATOMIC),
    LintId::of(mutex_atomic::MUTEX_INTEGER),
    LintId::of(non_send_fields_in_send_ty::NON_SEND_FIELDS_IN_SEND_TY),
    LintId::of(nonstandard_macro_braces::NONSTANDARD_MACRO_BRACES),
    LintId::of(option_if_let_else::OPTION_IF_LET_ELSE),
    LintId::of(path_buf_push_overwrite::PATH_BUF_PUSH_OVERWRITE),
    LintId::of(redundant_pub_crate::REDUNDANT_PUB_CRATE),
    LintId::of(regex::TRIVIAL_REGEX),
    LintId::of(strings::STRING_LIT_AS_BYTES),
    LintId::of(suspicious_operation_groupings::SUSPICIOUS_OPERATION_GROUPINGS),
    LintId::of(trailing_empty_array::TRAILING_EMPTY_ARRAY),
    LintId::of(transmute::USELESS_TRANSMUTE),
    LintId::of(use_self::USE_SELF),
])
