/*! Tokay builtin registry

THIS MODULE IS AUTOMATICALLY GENERATED BY BUILD.RS;
DON'T CHANGE THIS FILE MANUALLY, IT WILL GO AWAY!!!
*/
use crate::builtin::Builtin;

pub static BUILTINS: [Builtin; 64] = [
    Builtin {
        name: "Identifier",
        func: crate::value::token::tokay_token_identifier,
    },
    Builtin {
        name: "Integer",
        func: crate::value::token::tokay_token_integer,
    },
    Builtin {
        name: "Word",
        func: crate::value::token::tokay_token_word,
    },
    Builtin {
        name: "addr",
        func: crate::value::value::Value::tokay_method_addr,
    },
    Builtin {
        name: "addr_add",
        func: crate::value::value::Value::tokay_method_addr_add,
    },
    Builtin {
        name: "addr_div",
        func: crate::value::value::Value::tokay_method_addr_div,
    },
    Builtin {
        name: "addr_eq",
        func: crate::value::value::Value::tokay_method_addr_eq,
    },
    Builtin {
        name: "addr_gt",
        func: crate::value::value::Value::tokay_method_addr_gt,
    },
    Builtin {
        name: "addr_gteq",
        func: crate::value::value::Value::tokay_method_addr_gteq,
    },
    Builtin {
        name: "addr_lt",
        func: crate::value::value::Value::tokay_method_addr_lt,
    },
    Builtin {
        name: "addr_lteq",
        func: crate::value::value::Value::tokay_method_addr_lteq,
    },
    Builtin {
        name: "addr_mul",
        func: crate::value::value::Value::tokay_method_addr_mul,
    },
    Builtin {
        name: "addr_sub",
        func: crate::value::value::Value::tokay_method_addr_sub,
    },
    Builtin {
        name: "ast",
        func: crate::compiler::ast::tokay_function_ast,
    },
    Builtin {
        name: "ast_print",
        func: crate::compiler::ast::tokay_function_ast_print,
    },
    Builtin {
        name: "bool",
        func: crate::value::value::Value::tokay_method_bool,
    },
    Builtin {
        name: "chr",
        func: crate::builtin::tokay_function_chr,
    },
    Builtin {
        name: "dict",
        func: crate::value::dict::Dict::tokay_method_dict,
    },
    Builtin {
        name: "dict_update",
        func: crate::value::dict::Dict::tokay_method_dict_update,
    },
    Builtin {
        name: "error",
        func: crate::error::tokay_function_error,
    },
    Builtin {
        name: "float",
        func: crate::value::value::Value::tokay_method_float,
    },
    Builtin {
        name: "float_add",
        func: crate::value::value::Value::tokay_method_float_add,
    },
    Builtin {
        name: "float_div",
        func: crate::value::value::Value::tokay_method_float_div,
    },
    Builtin {
        name: "float_eq",
        func: crate::value::value::Value::tokay_method_float_eq,
    },
    Builtin {
        name: "float_gt",
        func: crate::value::value::Value::tokay_method_float_gt,
    },
    Builtin {
        name: "float_gteq",
        func: crate::value::value::Value::tokay_method_float_gteq,
    },
    Builtin {
        name: "float_lt",
        func: crate::value::value::Value::tokay_method_float_lt,
    },
    Builtin {
        name: "float_lteq",
        func: crate::value::value::Value::tokay_method_float_lteq,
    },
    Builtin {
        name: "float_mul",
        func: crate::value::value::Value::tokay_method_float_mul,
    },
    Builtin {
        name: "float_neg",
        func: crate::value::value::Value::tokay_method_float_neg,
    },
    Builtin {
        name: "float_sub",
        func: crate::value::value::Value::tokay_method_float_sub,
    },
    Builtin {
        name: "int",
        func: crate::value::value::Value::tokay_method_int,
    },
    Builtin {
        name: "int_add",
        func: crate::value::value::Value::tokay_method_int_add,
    },
    Builtin {
        name: "int_div",
        func: crate::value::value::Value::tokay_method_int_div,
    },
    Builtin {
        name: "int_eq",
        func: crate::value::value::Value::tokay_method_int_eq,
    },
    Builtin {
        name: "int_gt",
        func: crate::value::value::Value::tokay_method_int_gt,
    },
    Builtin {
        name: "int_gteq",
        func: crate::value::value::Value::tokay_method_int_gteq,
    },
    Builtin {
        name: "int_lt",
        func: crate::value::value::Value::tokay_method_int_lt,
    },
    Builtin {
        name: "int_lteq",
        func: crate::value::value::Value::tokay_method_int_lteq,
    },
    Builtin {
        name: "int_mul",
        func: crate::value::value::Value::tokay_method_int_mul,
    },
    Builtin {
        name: "int_neg",
        func: crate::value::value::Value::tokay_method_int_neg,
    },
    Builtin {
        name: "int_sub",
        func: crate::value::value::Value::tokay_method_int_sub,
    },
    Builtin {
        name: "list",
        func: crate::value::list::List::tokay_method_list,
    },
    Builtin {
        name: "list_add",
        func: crate::value::list::List::tokay_method_list_add,
    },
    Builtin {
        name: "list_eq",
        func: crate::value::list::List::tokay_method_list_eq,
    },
    Builtin {
        name: "list_gt",
        func: crate::value::list::List::tokay_method_list_gt,
    },
    Builtin {
        name: "list_gteq",
        func: crate::value::list::List::tokay_method_list_gteq,
    },
    Builtin {
        name: "list_iadd",
        func: crate::value::list::List::tokay_method_list_iadd,
    },
    Builtin {
        name: "list_lt",
        func: crate::value::list::List::tokay_method_list_lt,
    },
    Builtin {
        name: "list_lteq",
        func: crate::value::list::List::tokay_method_list_lteq,
    },
    Builtin {
        name: "list_push",
        func: crate::value::list::List::tokay_method_list_push,
    },
    Builtin {
        name: "ord",
        func: crate::builtin::tokay_function_ord,
    },
    Builtin {
        name: "print",
        func: crate::builtin::tokay_function_print,
    },
    Builtin {
        name: "repr",
        func: crate::builtin::tokay_function_repr,
    },
    Builtin {
        name: "str",
        func: crate::value::str::Str::tokay_method_str,
    },
    Builtin {
        name: "str_add",
        func: crate::value::str::Str::tokay_method_str_add,
    },
    Builtin {
        name: "str_endswith",
        func: crate::value::str::Str::tokay_method_str_endswith,
    },
    Builtin {
        name: "str_join",
        func: crate::value::str::Str::tokay_method_str_join,
    },
    Builtin {
        name: "str_lower",
        func: crate::value::str::Str::tokay_method_str_lower,
    },
    Builtin {
        name: "str_mul",
        func: crate::value::str::Str::tokay_method_str_mul,
    },
    Builtin {
        name: "str_replace",
        func: crate::value::str::Str::tokay_method_str_replace,
    },
    Builtin {
        name: "str_startswith",
        func: crate::value::str::Str::tokay_method_str_startswith,
    },
    Builtin {
        name: "str_upper",
        func: crate::value::str::Str::tokay_method_str_upper,
    },
    Builtin {
        name: "type",
        func: crate::builtin::tokay_function_type,
    },
];
