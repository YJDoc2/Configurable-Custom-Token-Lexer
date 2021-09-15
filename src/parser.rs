// auto-generated: "lalrpop 0.19.6"
// sha3: 8a5bc6d7a989c2aaad6c4682fb9b2229ca095656848947946e5ded733a4571
use crate::lexer;
use crate::tokens;
use crate::config::LangConfig;
use crate::ast::*;
use tokens::Token::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::lexer;
    use crate::tokens;
    use crate::config::LangConfig;
    use crate::ast::*;
    use tokens::Token::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(tokens::Token),
        Variant1(core::option::Option<tokens::Token>),
        Variant2(AST),
        Variant3(Stmt),
        Variant4(Block),
        Variant5(core::option::Option<Block>),
        Variant6(Condition),
        Variant7(Expr),
        Variant8(Vec<Stmt>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 58, 0, 59, 0, 0, 60, 0, 0, 0, 61,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 58, 0, 59, 0, 0, 60, 0, 0, 0, 61,
        // State 2
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 3
        8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 4
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 5
        8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 6
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 7
        8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 8
        8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 9
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 10
        8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 11
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 12
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 13
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 14
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 15
        8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 18
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 19
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 20
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 21
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 22
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 23
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -69, 32, 33, 0, 0, 0, 0, 0, -69, 0, 0, -69, 0, -69, 0, 0, -69, 0, 0, 0, -69,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 58, 0, 59, 0, 0, 60, 0, 0, 0, 61,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -53, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -61, 39, 0, 0, 0, 0, 0, 0, -61, 0, 0, -61, 0, -61, 0, 0, -61, 0, 0, 0, -61,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 58, 0, 59, 0, 0, 60, 0, 0, 0, 61,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -67, 40, 41, 0, 0, 0, 0, 0, -67, 0, 0, -67, 0, -67, 0, 0, -67, 0, 0, 0, -67,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -65, 42, 43, 0, 0, 0, 0, 0, -65, 0, 0, -65, 0, -65, 0, 0, -65, 0, 0, 0, -65,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -45, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, -45, 0, -45, 0, 0, -45, 0, 0, 0, -45,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -51, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, -51, 0, -51, 0, 0, -51, 0, 0, 0, -51,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -59, 45, 0, 0, 0, 0, 0, 0, -59, 0, 0, -59, 0, -59, 0, 0, -59, 0, 0, 0, -59,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -49, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, -49, 0, -49, 0, 0, -49, 0, 0, 0, -49,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -57, 46, 0, 0, 0, 0, 0, 0, -57, 0, 0, -57, 0, -57, 0, 0, -57, 0, 0, 0, -57,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -63, 47, 48, 0, 0, 0, 0, 0, -63, 0, 0, -63, 0, -63, 0, 0, -63, 0, 0, 0, -63,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -43, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, -43, 0, -43, 0, 0, -43, 0, 0, 0, -43,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -41, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, -41, 0, -41, 0, 0, -41, 0, 0, 0, -41,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -47, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, -47, 0, -47, 0, 0, -47, 0, 0, 0, -47,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -55, 49, 0, 0, 0, 0, 0, 0, -55, 0, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, -39, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, -39, 0, -39, 0, 0, -39, 0, 0, 0, -39,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, -83, 0, -83, 0, 0, -83, 0, 0, 0, -83,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, -80, 0, -80, 0, 0, -80, 0, 0, 0, -80,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, -79, 0, -79, 0, 0, -79, 0, 0, 0, -79,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, -77, 0, -77, 0, 0, -77, 0, 0, 0, -77,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, -84, 0, -84, 0, 0, -84, 0, 0, 0, -84,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, -81, 0, -81, 0, 0, -81, 0, 0, 0, -81,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, -82, 0, -82, 0, 0, -82, 0, 0, 0, -82,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, -78, 0, -78, 0, 0, -78, 0, 0, 0, -78,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, -76, 0, -76, 0, 0, -76, 0, 0, 0, -76,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, -85, 0, -85, 0, 0, -85, 0, 0, 0, -85,
        // State 65
        0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 12, 0, 13, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, -18, 0, -18, 0, 0, -18, 0, 0, 0, -18,
        // State 69
        0, -33, -33, 0, -33, 14, -33, 0, -33, 15, -33, -33, -33, 0, -33, -33, -33, 0, -33, -33, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, -33, 0, -33, 0, 0, -33, 0, 0, 0, -33,
        // State 70
        0, -90, -90, 0, -90, -90, -90, 0, -90, -90, -90, -90, -90, 0, -90, -90, -90, 0, -90, -90, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, -90, 0, -90, 0, 0, -90, 0, 0, 0, -90,
        // State 71
        0, -89, -89, 0, -89, -89, -89, 0, -89, -89, -89, -89, -89, 0, -89, -89, -89, 0, -89, -89, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, -89, 0, -89, 0, 0, -89, 0, 0, 0, -89,
        // State 72
        0, -91, -91, 0, -91, -91, -91, 0, -91, -91, -91, -91, -91, 0, -91, -91, -91, 0, -91, -91, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, -91, 0, -91, 0, 0, -91, 0, 0, 0, -91,
        // State 73
        0, -36, -36, 0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, 0, -36, -36, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, -36, 0, -36, 0, 0, -36, 0, 0, 0, -36,
        // State 74
        0, -37, -37, 0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 0, -37, -37, -37, 0, -37, -37, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, -37, 0, -37, 0, 0, -37, 0, 0, 0, -37,
        // State 75
        0, -71, -71, 0, -71, -71, -71, 0, -71, -71, -71, -71, -71, 0, -71, -71, -71, 0, -71, -71, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, -71, 0, -71, 0, 0, -71, 0, 0, 0, -71,
        // State 76
        0, -86, -86, 0, -86, -86, -86, 0, -86, -86, -86, -86, -86, 0, -86, -86, -86, 0, -86, -86, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, -86, 0, -86, 0, 0, -86, 0, 0, 0, -86,
        // State 77
        0, 0, -87, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 16, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 19, 0, 0, 0, 0, 12, 0, 13, 0, 0, 20, 21, 0, 22, 23, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 88, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 16, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 94, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, -88, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 16, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 12, 0, 13, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, -70, 0, -70, 0, 0, -70, 0, 0, 0, -70,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, -74, 0, -74, 0, 103, -74, 0, 0, 0, -74,
        // State 88
        0, 0, 16, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, -31, -31, 0, -31, 14, -31, 0, -31, 15, -31, -31, -31, 0, -31, -31, -31, 0, -31, -31, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, -31, 0, -31, 0, 0, -31, 0, 0, 0, -31,
        // State 90
        0, -32, -32, 0, -32, 14, -32, 0, -32, 15, -32, -32, -32, 0, -32, -32, -32, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, -32, 0, -32, 0, 0, -32, 0, 0, 0, -32,
        // State 91
        0, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34, -34, -34, 0, -34, -34, -34, 0, -34, -34, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, -34, 0, -34, 0, 0, -34, 0, 0, 0, -34,
        // State 92
        0, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, 0, -35, -35, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, -35, 0, -35, 0, 0, -35, 0, 0, 0, -35,
        // State 93
        0, -72, -72, 0, -72, -72, -72, 0, -72, -72, -72, -72, -72, 0, -72, -72, -72, 0, -72, -72, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, -72, 0, -72, 0, 0, -72, 0, 0, 0, -72,
        // State 94
        0, 0, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, 0, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        0, 0, -26, 0, -26, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 0, -22, 0, -22, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, -24, 0, -24, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, 0, -27, 0, -27, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, 0, -23, 0, -23, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 0, -25, 0, -25, 0, 12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, -73, 0, -73, 0, 0, -73, 0, 0, 0, -73,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, -95, 0, -95, 0, 0, -95, 0, 0, 0, -95,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, -68, 0, -68, 0, 0, -68, 0, 0, 0, -68,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, -94, 0, -94, 0, 0, -94, 0, 0, 0, -94,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, -93, 0, -93, 0, 0, -93, 0, 0, 0, -93,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, -52, 0, -52, 0, 0, -52, 0, 0, 0, -52,
        // State 108
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, -60, 0, -60, 0, 0, -60, 0, 0, 0, -60,
        // State 109
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, -19, -19, -19, 0, 0, 0, 0, 0, -19, 0, 0, -19, 0, -19, 0, 0, -19, 0, 0, 0, -19,
        // State 110
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, -66, 0, -66, 0, 0, -66, 0, 0, 0, -66,
        // State 111
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, -64, 0, -64, 0, 0, -64, 0, 0, 0, -64,
        // State 112
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, -92, 0, -92, 0, 0, -92, 0, 0, 0, -92,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, -44, 0, -44, 0, 0, -44, 0, 0, 0, -44,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, -50, 0, -50, 0, 0, -50, 0, 0, 0, -50,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, -58, 0, -58, 0, 0, -58, 0, 0, 0, -58,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, -48, 0, -48, 0, 0, -48, 0, 0, 0, -48,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, -62, 0, -62, 0, 0, -62, 0, 0, 0, -62,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, -42, 0, -42, 0, 0, -42, 0, 0, 0, -42,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, -40, 0, -40, 0, 0, -40, 0, 0, 0, -40,
        // State 121
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, -46, 0, -46, 0, 0, -46, 0, 0, 0, -46,
        // State 122
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54,
        // State 123
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, -38, 0, -38, 0, 0, -38, 0, 0, 0, -38,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 40 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -75,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -69,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        -53,
        // State 32
        -61,
        // State 33
        0,
        // State 34
        -67,
        // State 35
        -65,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -45,
        // State 39
        -51,
        // State 40
        -59,
        // State 41
        -49,
        // State 42
        -57,
        // State 43
        -63,
        // State 44
        -43,
        // State 45
        -41,
        // State 46
        -47,
        // State 47
        -55,
        // State 48
        -39,
        // State 49
        -83,
        // State 50
        -80,
        // State 51
        -79,
        // State 52
        -77,
        // State 53
        -17,
        // State 54
        -84,
        // State 55
        -81,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        -82,
        // State 62
        -78,
        // State 63
        -76,
        // State 64
        -85,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        -18,
        // State 69
        -33,
        // State 70
        -90,
        // State 71
        -89,
        // State 72
        -91,
        // State 73
        -36,
        // State 74
        -37,
        // State 75
        -71,
        // State 76
        -86,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        -70,
        // State 87
        -74,
        // State 88
        0,
        // State 89
        -31,
        // State 90
        -32,
        // State 91
        -34,
        // State 92
        -35,
        // State 93
        -72,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        -73,
        // State 103
        -95,
        // State 104
        -68,
        // State 105
        -94,
        // State 106
        -93,
        // State 107
        -52,
        // State 108
        -60,
        // State 109
        -19,
        // State 110
        -66,
        // State 111
        -64,
        // State 112
        -92,
        // State 113
        -44,
        // State 114
        -50,
        // State 115
        -58,
        // State 116
        -48,
        // State 117
        -56,
        // State 118
        -62,
        // State 119
        -42,
        // State 120
        -40,
        // State 121
        -46,
        // State 122
        -54,
        // State 123
        -38,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            9 => 49,
            10 => match state {
                27 => 34,
                28 => 35,
                36 => 43,
                24 => 103,
                25 => 104,
                29 => 105,
                30 => 106,
                31 => 107,
                32 => 108,
                34 => 110,
                35 => 111,
                37 => 112,
                38 => 113,
                39 => 114,
                40 => 115,
                41 => 116,
                42 => 117,
                43 => 118,
                44 => 119,
                45 => 120,
                46 => 121,
                47 => 122,
                48 => 123,
                _ => 25,
            },
            12 => 77,
            13 => match state {
                5 => 82,
                8 => 85,
                10 => 88,
                _ => 78,
            },
            14 => match state {
                2 => 68,
                4 => 81,
                6 => 83,
                9 => 86,
                18 => 96,
                19 => 97,
                20 => 98,
                21 => 99,
                22 => 100,
                23 => 101,
                _ => 79,
            },
            15 => match state {
                11 => 89,
                12 => 90,
                _ => 69,
            },
            16 => 70,
            17 => 50,
            18 => 51,
            19 => 71,
            20 => 52,
            21 => 53,
            22 => match state {
                1 | 33 => 64,
                _ => 54,
            },
            23 => match state {
                26 => 33,
                _ => 1,
            },
            24 => 72,
            25 => match state {
                7 => 84,
                15 => 94,
                17 => 95,
                _ => 80,
            },
            26 => match state {
                13 => 91,
                14 => 92,
                _ => 73,
            },
            27 => 55,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
            r###"ElseAux1"###,
            r###"ElseStart"###,
            r###"ForAux1"###,
            r###"ForAux2"###,
            r###"ForAux3"###,
            r###"ForAux4"###,
            r###"ForStart"###,
            r###"ID"###,
            r###"IfAux1"###,
            r###"IfAux2"###,
            r###"IfStart"###,
            r###"In"###,
            r###"LetStart"###,
            r###"Num"###,
            r###"PrintEnd"###,
            r###"PrintStart"###,
            r###"StrLit"###,
            r###"WhileAux1"###,
            r###"WhileAux2"###,
            r###"WhileStart"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'__0>
    where 
    {
        lc: &'__0 LangConfig,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<'__0> __state_machine::ParserDefinition for __StateMachine<'__0>
    where 
    {
        type Location = usize;
        type Error = lexer::LexerError;
        type Token = tokens::Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = AST;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 40 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.lc,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &tokens::Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Not if true => Some(0),
            NotEq if true => Some(1),
            And if true => Some(2),
            OpenRoundBrack if true => Some(3),
            CloseRoundBrack if true => Some(4),
            Mul if true => Some(5),
            Plus if true => Some(6),
            Comma if true => Some(7),
            Minus if true => Some(8),
            Div if true => Some(9),
            SemiColon if true => Some(10),
            LessThan if true => Some(11),
            LessThanEq if true => Some(12),
            Eq if true => Some(13),
            EqEq if true => Some(14),
            GreaterThan if true => Some(15),
            GreaterThanEq if true => Some(16),
            OpenCurlyBrack if true => Some(17),
            Or if true => Some(18),
            CloseCurlyBrack if true => Some(19),
            ElseAux1 if true => Some(20),
            ElseStart if true => Some(21),
            ForAux1 if true => Some(22),
            ForAux2 if true => Some(23),
            ForAux3 if true => Some(24),
            ForAux4 if true => Some(25),
            ForStart if true => Some(26),
            ID(_) if true => Some(27),
            IfAux1 if true => Some(28),
            IfAux2 if true => Some(29),
            IfStart if true => Some(30),
            In if true => Some(31),
            LetStart if true => Some(32),
            Number(_) if true => Some(33),
            PrintEnd if true => Some(34),
            PrintStart if true => Some(35),
            StringVal(_) if true => Some(36),
            WhileAux1 if true => Some(37),
            WhileAux2 if true => Some(38),
            WhileStart if true => Some(39),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: tokens::Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    pub struct programParser {
        _priv: (),
    }

    impl programParser {
        pub fn new() -> programParser {
            programParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            lc: &LangConfig,
            __tokens0: __TOKENS,
        ) -> Result<AST, __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    lc,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        lc: &LangConfig,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<AST,__lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(lc, __sym0);
                return Some(Ok(__nt));
            }
            17 => {
                __reduce17(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            79 => {
                __reduce79(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            80 => {
                __reduce80(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            81 => {
                __reduce81(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            82 => {
                __reduce82(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            83 => {
                __reduce83(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            84 => {
                __reduce84(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            85 => {
                __reduce85(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            86 => {
                __reduce86(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            87 => {
                __reduce87(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            88 => {
                __reduce88(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            89 => {
                __reduce89(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            90 => {
                __reduce90(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            91 => {
                __reduce91(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            92 => {
                __reduce92(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            93 => {
                __reduce93(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            94 => {
                __reduce94(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AST, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Block, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Condition, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Stmt>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<Block>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<tokens::Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, tokens::Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? = ";" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ElseAux1? = ElseAux1 => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ElseAux1? =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce4<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ElseStart? = ElseStart => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ElseStart? =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce6<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IfAux1? = IfAux1 => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce7<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IfAux1? =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action50::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce8<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IfAux2? = IfAux2 => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce9<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IfAux2? =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce10<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PrintEnd? = PrintEnd => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce11<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PrintEnd? =  => ActionFn(40);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action40::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce12<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // WhileAux1? = WhileAux1 => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // WhileAux1? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce14<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // WhileAux2? = WhileAux2 => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce15<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // WhileAux2? =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce17<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // assign_stmt = ID, "=", expr => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce18<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // block = "{", stmts, "}" => ActionFn(2);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action2::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce19<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // block? = block => ActionFn(41);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce20<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // block? =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action42::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce21<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // comparison = expr, "<", expr => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce22<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // comparison = expr, ">", expr => ActionFn(21);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce23<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // comparison = expr, "<=", expr => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action22::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce24<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // comparison = expr, ">=", expr => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // comparison = expr, "!=", expr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce26<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // comparison = expr, "==", expr => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce27<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // condition = condition, "&&", unary_condition => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce28<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // condition = condition, "||", unary_condition => ActionFn(16);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce29<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // condition = unary_condition => ActionFn(17);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce30<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // expr = expr, "+", factor => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce31<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // expr = expr, "-", factor => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action27::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce32<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // expr = factor => ActionFn(28);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce33<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // factor = factor, "*", value => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce34<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // factor = factor, "/", value => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce35<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // factor = value => ActionFn(31);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce36<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // id = ID => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce37<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block, ElseStart, ElseAux1, block => ActionFn(101);
        assert!(__symbols.len() >= 10);
        let __sym9 = __pop_Variant4(__symbols);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym9.2.clone();
        let __nt = super::__action101::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (10, 17)
    }
    pub(crate) fn __reduce38<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block, ElseStart, ElseAux1 => ActionFn(102);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action102::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (9, 17)
    }
    pub(crate) fn __reduce39<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block, ElseStart, ElseAux1, block => ActionFn(103);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant4(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action103::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (9, 17)
    }
    pub(crate) fn __reduce40<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block, ElseStart, ElseAux1 => ActionFn(104);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action104::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce41<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block, ElseStart, ElseAux1, block => ActionFn(105);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant4(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action105::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (9, 17)
    }
    pub(crate) fn __reduce42<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block, ElseStart, ElseAux1 => ActionFn(106);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action106::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce43<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block, ElseStart, ElseAux1, block => ActionFn(107);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant4(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action107::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce44<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block, ElseStart, ElseAux1 => ActionFn(108);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action108::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce45<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block, ElseAux1, block => ActionFn(109);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant4(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action109::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (9, 17)
    }
    pub(crate) fn __reduce46<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block, ElseAux1 => ActionFn(110);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action110::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce47<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block, ElseAux1, block => ActionFn(111);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant4(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action111::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce48<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block, ElseAux1 => ActionFn(112);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action112::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce49<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block, ElseAux1, block => ActionFn(113);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant4(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action113::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce50<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block, ElseAux1 => ActionFn(114);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action114::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce51<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block, ElseAux1, block => ActionFn(115);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action115::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce52<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block, ElseAux1 => ActionFn(116);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action116::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce53<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block, ElseStart, block => ActionFn(117);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant4(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action117::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (9, 17)
    }
    pub(crate) fn __reduce54<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block, ElseStart => ActionFn(118);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action118::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce55<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block, ElseStart, block => ActionFn(119);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant4(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action119::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce56<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block, ElseStart => ActionFn(120);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action120::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce57<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block, ElseStart, block => ActionFn(121);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant4(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action121::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce58<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block, ElseStart => ActionFn(122);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action122::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce59<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block, ElseStart, block => ActionFn(123);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action123::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce60<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block, ElseStart => ActionFn(124);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action124::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce61<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block, block => ActionFn(125);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant4(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action125::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (8, 17)
    }
    pub(crate) fn __reduce62<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", IfAux2, block => ActionFn(126);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action126::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce63<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block, block => ActionFn(127);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action127::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce64<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, IfAux1, "(", condition, ")", block => ActionFn(128);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action128::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce65<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block, block => ActionFn(129);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action129::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 17)
    }
    pub(crate) fn __reduce66<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", IfAux2, block => ActionFn(130);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action130::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce67<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block, block => ActionFn(131);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action131::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (6, 17)
    }
    pub(crate) fn __reduce68<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // if_stmt = IfStart, "(", condition, ")", block => ActionFn(132);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action132::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 17)
    }
    pub(crate) fn __reduce69<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // let_stmt = LetStart, ID, "=", expr => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(lc, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 18)
    }
    pub(crate) fn __reduce70<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // num = Num => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce71<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // num = "(", expr, ")" => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce72<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // print_stmt = PrintStart, "(", expr, ")", PrintEnd => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 20)
    }
    pub(crate) fn __reduce73<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // print_stmt = PrintStart, "(", expr, ")" => ActionFn(94);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action94::<>(lc, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 20)
    }
    pub(crate) fn __reduce74<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // program = stmts => ActionFn(1);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce75<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = print_stmt, ";" => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action57::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce76<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = print_stmt => ActionFn(58);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce77<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = let_stmt, ";" => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce78<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = let_stmt => ActionFn(60);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce79<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = if_stmt => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce80<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = while_stmt => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce81<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = assign_stmt, ";" => ActionFn(61);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action61::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce82<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmt = assign_stmt => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce83<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = stmt => ActionFn(8);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce84<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = stmts, stmt => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce85<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // str_lit = StrLit => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce86<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // unary_condition = comparison => ActionFn(18);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce87<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // unary_condition = "!", unary_condition => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 25)
    }
    pub(crate) fn __reduce88<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // value = num => ActionFn(32);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce89<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // value = id => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce90<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // value = str_lit => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce91<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // while_stmt = WhileStart, WhileAux1, "(", condition, ")", WhileAux2, block => ActionFn(97);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action97::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (7, 27)
    }
    pub(crate) fn __reduce92<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // while_stmt = WhileStart, WhileAux1, "(", condition, ")", block => ActionFn(98);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action98::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (6, 27)
    }
    pub(crate) fn __reduce93<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // while_stmt = WhileStart, "(", condition, ")", WhileAux2, block => ActionFn(99);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action99::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (6, 27)
    }
    pub(crate) fn __reduce94<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // while_stmt = WhileStart, "(", condition, ")", block => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 27)
    }
}
pub use self::__parse__program::programParser;

#[allow(unused_variables)]
fn __action0<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, AST, usize),
) -> AST
{
    __0
}

#[allow(unused_variables)]
fn __action1<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Vec<Stmt>, usize),
) -> AST
{
    AST{top_block:Block{stmts:__0}}
}

#[allow(unused_variables)]
fn __action2<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, __0, _): (usize, Vec<Stmt>, usize),
    (_, _, _): (usize, tokens::Token, usize),
) -> Block
{
    Block{stmts:__0}
}

#[allow(unused_variables)]
fn __action3<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Stmt, usize),
    (_, _, _): (usize, core::option::Option<tokens::Token>, usize),
) -> Stmt
{
    __0
}

#[allow(unused_variables)]
fn __action4<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Stmt, usize),
    (_, _, _): (usize, core::option::Option<tokens::Token>, usize),
) -> Stmt
{
    __0
}

#[allow(unused_variables)]
fn __action5<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Stmt, usize),
) -> Stmt
{
    __0
}

#[allow(unused_variables)]
fn __action6<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Stmt, usize),
) -> Stmt
{
    __0
}

#[allow(unused_variables)]
fn __action7<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Stmt, usize),
    (_, _, _): (usize, core::option::Option<tokens::Token>, usize),
) -> Stmt
{
    __0
}

#[allow(unused_variables)]
fn __action8<
>(
    lc: &LangConfig,
    (_, s, _): (usize, Stmt, usize),
) -> Vec<Stmt>
{
    vec![s]
}

#[allow(unused_variables)]
fn __action9<
>(
    lc: &LangConfig,
    (_, mut s, _): (usize, Vec<Stmt>, usize),
    (_, n, _): (usize, Stmt, usize),
) -> Vec<Stmt>
{
    {s.push(n);s}
}

#[allow(unused_variables)]
fn __action10<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, wa1, _): (usize, core::option::Option<tokens::Token>, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, c, _): (usize, Condition, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, wa2, _): (usize, core::option::Option<tokens::Token>, usize),
    (_, b, _): (usize, Block, usize),
) -> Stmt
{
    {
        if !lc.is_optional(WhileAux1) && wa1.is_none(){
            panic!("Error, while aux 1 required");
        }
        if !lc.is_optional(WhileAux2) && wa2.is_none(){
            panic!("Error, while aux 2 required");
        }
        Stmt::While(While{cond:c,block:b})
    }
}

#[allow(unused_variables)]
fn __action11<
>(
    lc: &LangConfig,
    (_, name, _): (usize, tokens::Token, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> Stmt
{
    {
        if let ID(name) = name{
            Stmt::Assign(Assign{id:name,value:e})
        }else{
            unreachable!();
        }
    }
}

#[allow(unused_variables)]
fn __action12<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, ifa1, _): (usize, core::option::Option<tokens::Token>, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, c, _): (usize, Condition, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, ifa2, _): (usize, core::option::Option<tokens::Token>, usize),
    (_, b1, _): (usize, Block, usize),
    (_, els, _): (usize, core::option::Option<tokens::Token>, usize),
    (_, ea1, _): (usize, core::option::Option<tokens::Token>, usize),
    (_, b2, _): (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    {
        if !lc.is_optional(IfAux1) && ifa1.is_none(){
            panic!("Error, if aux 1 required");
        }
        if !lc.is_optional(IfAux2) && ifa2.is_none(){
            panic!("Error, if aux 2 required");
        }
        
        // else section
        let else_blk;

        if els.is_some(){
            if !lc.is_optional(ElseAux1) && ea1.is_none(){
                panic!("Error, else aux 1 required");
            }
            if b2.is_none(){
                panic!("A block is required for else statements");
            }
            else_blk = b2.unwrap();
        }else{
            else_blk = Block{stmts:vec![]};
        }
        return Stmt::If(IfStmt{cond:c,if_blk:b1,else_blk:else_blk});
    }
}

#[allow(unused_variables)]
fn __action13<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, v, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e, _): (usize, core::option::Option<tokens::Token>, usize),
) -> Stmt
{
    {
        if !lc.is_optional(PrintEnd) && e.is_none(){
            panic!("Error, print end required");
        }
        return Stmt::Print(PrintStmt{arg:v});
    }
}

#[allow(unused_variables)]
fn __action14<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, name, _): (usize, tokens::Token, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, v, _): (usize, Expr, usize),
) -> Stmt
{
    {
        if let ID(name) = name{
            return Stmt::Decl(Decl{name:name,value:v});
        }else{
            unreachable!();
        }
        
    }
}

#[allow(unused_variables)]
fn __action15<
>(
    lc: &LangConfig,
    (_, c1, _): (usize, Condition, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, c2, _): (usize, Condition, usize),
) -> Condition
{
    Condition::LogicalOp(Box::new(c1),LogicalOp::And,Box::new(c2))
}

#[allow(unused_variables)]
fn __action16<
>(
    lc: &LangConfig,
    (_, c1, _): (usize, Condition, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, c2, _): (usize, Condition, usize),
) -> Condition
{
    Condition::LogicalOp(Box::new(c1),LogicalOp::Or,Box::new(c2))
}

#[allow(unused_variables)]
fn __action17<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Condition, usize),
) -> Condition
{
    __0
}

#[allow(unused_variables)]
fn __action18<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Condition, usize),
) -> Condition
{
    __0
}

#[allow(unused_variables)]
fn __action19<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, c, _): (usize, Condition, usize),
) -> Condition
{
    Condition::Not(Box::new(c))
}

#[allow(unused_variables)]
fn __action20<
>(
    lc: &LangConfig,
    (_, e1, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e2, _): (usize, Expr, usize),
) -> Condition
{
    Condition::Comparison(e1,ComparisonOp::LT,e2)
}

#[allow(unused_variables)]
fn __action21<
>(
    lc: &LangConfig,
    (_, e1, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e2, _): (usize, Expr, usize),
) -> Condition
{
    Condition::Comparison(e1,ComparisonOp::GT,e2)
}

#[allow(unused_variables)]
fn __action22<
>(
    lc: &LangConfig,
    (_, e1, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e2, _): (usize, Expr, usize),
) -> Condition
{
    Condition::Comparison(e1,ComparisonOp::LTE,e2)
}

#[allow(unused_variables)]
fn __action23<
>(
    lc: &LangConfig,
    (_, e1, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e2, _): (usize, Expr, usize),
) -> Condition
{
    Condition::Comparison(e1,ComparisonOp::GTE,e2)
}

#[allow(unused_variables)]
fn __action24<
>(
    lc: &LangConfig,
    (_, e1, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e2, _): (usize, Expr, usize),
) -> Condition
{
    Condition::Comparison(e1,ComparisonOp::NotEq,e2)
}

#[allow(unused_variables)]
fn __action25<
>(
    lc: &LangConfig,
    (_, e1, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e2, _): (usize, Expr, usize),
) -> Condition
{
    Condition::Comparison(e1,ComparisonOp::Eq,e2)
}

#[allow(unused_variables)]
fn __action26<
>(
    lc: &LangConfig,
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, f, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(e),Operator::Add,Box::new(f))
}

#[allow(unused_variables)]
fn __action27<
>(
    lc: &LangConfig,
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, f, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(e),Operator::Sub,Box::new(f))
}

#[allow(unused_variables)]
fn __action28<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action29<
>(
    lc: &LangConfig,
    (_, f, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, n, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(f),Operator::Mul,Box::new(n))
}

#[allow(unused_variables)]
fn __action30<
>(
    lc: &LangConfig,
    (_, f, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, n, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(f),Operator::Div,Box::new(n))
}

#[allow(unused_variables)]
fn __action31<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action32<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action33<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action34<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action35<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> Expr
{
    {
        if let ID(t) = __0 {
            return Expr::Val(Value::Id(t));
        }else{
            return Expr::Val(Value::Id("".to_owned()));
        }
    }
}

#[allow(unused_variables)]
fn __action36<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> Expr
{
    {
        if let StringVal(t) = __0 {
            return Expr::Val(Value::Sval(t));
        }else{
            return Expr::Val(Value::Sval("".to_owned()));
        }
    }
}

#[allow(unused_variables)]
fn __action37<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> Expr
{
    {
        if let Number(t) = __0 {
            return Expr::Val(Value::Nval(t.parse().unwrap()));
        }else{
            return Expr::Val(Value::Nval(0.0));
        }
    }
}

#[allow(unused_variables)]
fn __action38<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action39<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action40<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action41<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Block, usize),
) -> core::option::Option<Block>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action42<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Block>
{
    None
}

#[allow(unused_variables)]
fn __action43<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action44<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action45<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action46<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action47<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action48<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action49<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action50<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action51<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action52<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action53<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action54<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action55<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action56<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action57<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
    __1: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action55(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action58<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action59<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
    __1: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action55(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
    __1: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action55(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, core::option::Option<tokens::Token>, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, core::option::Option<tokens::Token>, usize),
    __8: (usize, tokens::Token, usize),
    __9: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __8.0.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action43(
        lc,
        __8,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
        __9,
    )
}

#[allow(unused_variables)]
fn __action64<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, core::option::Option<tokens::Token>, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, core::option::Option<tokens::Token>, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __7.2.clone();
    let __end0 = __8.0.clone();
    let __temp0 = __action44(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
        __8,
    )
}

#[allow(unused_variables)]
fn __action65<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, core::option::Option<tokens::Token>, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, tokens::Token, usize),
    __9: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action45(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __8,
        __9,
    )
}

#[allow(unused_variables)]
fn __action66<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, core::option::Option<tokens::Token>, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __7.0.clone();
    let __temp0 = __action46(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action67<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, core::option::Option<tokens::Token>, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action45(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __8,
    )
}

#[allow(unused_variables)]
fn __action68<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, core::option::Option<tokens::Token>, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __7.0.clone();
    let __temp0 = __action46(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __7,
    )
}

#[allow(unused_variables)]
fn __action69<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, tokens::Token, usize),
    __9: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        lc,
        __0,
        __temp0,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
        __9,
    )
}

#[allow(unused_variables)]
fn __action70<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, core::option::Option<tokens::Token>, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action50(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        lc,
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action71<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        lc,
        __0,
        __temp0,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action72<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, core::option::Option<tokens::Token>, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action50(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        lc,
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action73<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        lc,
        __0,
        __temp0,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action74<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, core::option::Option<tokens::Token>, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action50(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        lc,
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action75<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        lc,
        __0,
        __temp0,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action76<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, core::option::Option<tokens::Token>, usize),
    __5: (usize, Block, usize),
    __6: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action50(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        lc,
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action77<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, tokens::Token, usize),
    __9: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action47(
        lc,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
        __8,
        __9,
    )
}

#[allow(unused_variables)]
fn __action78<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action79<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action47(
        lc,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action80<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action81<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action47(
        lc,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action82<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action83<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action47(
        lc,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action84<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action85<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action47(
        lc,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
fn __action86<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action87<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action47(
        lc,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action88<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action89<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action47(
        lc,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action90<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action91<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action47(
        lc,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action92<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, core::option::Option<Block>, usize),
) -> Stmt
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action48(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action93<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Expr, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action39(
        lc,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action94<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Expr, usize),
    __3: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action40(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action95<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, core::option::Option<tokens::Token>, usize),
    __6: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action53(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        lc,
        __0,
        __temp0,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action96<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, core::option::Option<tokens::Token>, usize),
    __5: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action54(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        lc,
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action97<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action51(
        lc,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
fn __action98<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action52(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
fn __action99<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action51(
        lc,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
fn __action100<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action52(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
fn __action101<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, tokens::Token, usize),
    __9: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __9.0.clone();
    let __end0 = __9.2.clone();
    let __temp0 = __action41(
        lc,
        __9,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action102<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __8.2.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __8,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action103<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __8.0.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action41(
        lc,
        __8,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action104<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __7.2.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action105<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __8.0.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action41(
        lc,
        __8,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action106<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __7.2.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action41(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action108<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action109<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __8.0.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action41(
        lc,
        __8,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __7.2.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action111<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action41(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action112<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action113<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action41(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action114<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action115<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __6.0.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action41(
        lc,
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action116<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action117<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
    __8: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __8.0.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action41(
        lc,
        __8,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action118<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __7.2.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action119<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action41(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action120<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action121<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
    __7: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action41(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action122<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action123<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __6.0.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action41(
        lc,
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action124<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action125<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
    __7: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action41(
        lc,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action126<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, tokens::Token, usize),
    __6: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action127<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __6.0.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action41(
        lc,
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action128<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, tokens::Token, usize),
    __3: (usize, Condition, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action129<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
    __6: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __6.0.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action41(
        lc,
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action130<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
    __5: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action131<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
    __5: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action41(
        lc,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action132<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Condition, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, Block, usize),
) -> Stmt
{
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action42(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        lc,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,tokens::Token,usize), __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>>;
}

impl<> __ToTriple<> for (usize, tokens::Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,tokens::Token,usize), __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, tokens::Token, usize), lexer::LexerError> {
    fn to_triple(value: Self) -> Result<(usize,tokens::Token,usize), __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
