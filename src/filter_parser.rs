// auto-generated: "lalrpop 0.23.1"
// sha3: 49f17f02802fccf64c2823464bc705ee91878787e91d75a5e2251e4ee7460f9c
use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
#[allow(unused_extern_crates)]
extern crate alloc;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__Filter {

    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(AttrExp),
        Variant2(AttrPath),
        Variant3(()),
        Variant4(CompValue),
        Variant5(CompareOp),
        Variant6(Filter),
        Variant7(PatchPath),
        Variant8(ValFilter),
        Variant9(ValuePath),
    }
    const __ACTION: &[i8] = &[
        // State 0
        28, 29, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 31, 32, 33, 34, 35, 36, 37, 38, 0, 39, 40, 0, 0, 0, 30, 0, 0, 0, 0,
        // State 2
        28, 29, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 46, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 43, 44,
        // State 5
        28, 29, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        28, 29, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        28, 29, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0,
        // State 9
        28, 29, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 31, 32, 33, 34, 35, 36, 37, 38, 0, 39, 40, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        28, 29, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0,
        // State 16
        28, 29, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        28, 29, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        28, 29, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0,
        // State 26
        -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3, -3, 0, 0, 0, -3, 0, 0, 0, 0,
        // State 29
        -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14,
        // State 31
        0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12,
        // State 32
        0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 33
        0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19,
        // State 34
        0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 35
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20,
        // State 36
        0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 37
        0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13,
        // State 38
        0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0,
        // State 39
        0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 40
        0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, -2, 0, 0, -2, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, -8, 0, 0, -8, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, -9, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, -10, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, -11, 0, 0, -11, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, -40, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, -39, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, -37, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, -42, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, -35, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, -38, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
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
        -26,
        // State 21
        -44,
        // State 22
        -25,
        // State 23
        -22,
        // State 24
        -23,
        // State 25
        -27,
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
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -1,
        // State 39
        0,
        // State 40
        -2,
        // State 41
        -7,
        // State 42
        -8,
        // State 43
        -9,
        // State 44
        -10,
        // State 45
        -11,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -21,
        // State 51
        -24,
        // State 52
        -29,
        // State 53
        -5,
        // State 54
        -43,
        // State 55
        -6,
        // State 56
        -28,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                5 | 11 | 16..=18 => 46,
                _ => 20,
            },
            1 => match state {
                5 | 11 | 16..=18 => 10,
                _ => 1,
            },
            2 => match state {
                14 => 56,
                15 => 57,
                19 => 60,
                _ => 52,
            },
            3 => 54,
            4 => 40,
            5 => 4,
            6 => match state {
                9 => 14,
                0 => 21,
                _ => 8,
            },
            7 => match state {
                7 => 51,
                _ => 22,
            },
            8 => match state {
                6 => 50,
                _ => 23,
            },
            9 => 24,
            10 => match state {
                3 => 9,
                5 | 11 | 16..=18 => 11,
                13 => 18,
                _ => 2,
            },
            11 => 5,
            13 => match state {
                11 => 15,
                18 => 19,
                _ => 12,
            },
            14 => match state {
                17 => 59,
                _ => 47,
            },
            15 => match state {
                16 => 58,
                _ => 48,
            },
            16 => 49,
            17 => 25,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"AttrNameTok"###,
        r###"AttrPathTok"###,
        r###"StringLit"###,
        r###""not""###,
        r###"NumberLit"###,
        r###""and""###,
        r###""co""###,
        r###""eq""###,
        r###""ew""###,
        r###""ge""###,
        r###""gt""###,
        r###""le""###,
        r###""lt""###,
        r###""ne""###,
        r###""or""###,
        r###""pr""###,
        r###""sw""###,
        r###""(""###,
        r###"")""###,
        r###"".""###,
        r###""[""###,
        r###""]""###,
        r###""false""###,
        r###""null""###,
        r###""true""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'b,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'b>
    where 
    {
        budget: &'b ParseBudget,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'b ())>,
    }
    impl<'input, 'b> __state_machine::ParserDefinition for __StateMachine<'input, 'b>
    where 
    {
        type Location = usize;
        type Error = FilterActionError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Filter;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
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
                self.budget,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'b,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
            Token(8, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(10, _) if true => Some(10),
            Token(11, _) if true => Some(11),
            Token(12, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(14, _) if true => Some(14),
            Token(15, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            Token(17, _) if true => Some(17),
            Token(18, _) if true => Some(18),
            Token(19, _) if true => Some(19),
            Token(20, _) if true => Some(20),
            Token(21, _) if true => Some(21),
            Token(22, _) if true => Some(22),
            Token(23, _) if true => Some(23),
            Token(24, _) if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'b,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'b,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'b>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            43 => __state_machine::SimulatedReduce::Accept,
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct FilterParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for FilterParser { fn default() -> Self { Self::new() } }
    impl FilterParser {
        pub fn new() -> FilterParser {
            let __builder = super::__intern_token::new_builder();
            FilterParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'b,
        >(
            &self,
            budget: &'b ParseBudget,
            input: &'input str,
        ) -> Result<Filter, __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    budget,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'b,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<Result<Filter,__lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                // AttrExp = AttrPath, "pr" => ActionFn(21);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action21::<>(budget, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            1 => {
                // AttrExp = AttrPath, CompareOp, CompValue => ActionFn(22);
                assert!(__symbols.len() >= 3);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action22::<>(budget, input, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 0)
            }
            2 => {
                // AttrPath = AttrPathTok => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action23::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            3 => {
                __reduce3(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                // CompValue = NumberLit => ActionFn(45);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action45::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            10 => {
                // CompValue = StringLit => ActionFn(46);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            11 => {
                __reduce11(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                // Open = "(" => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action16::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 10)
            }
            30 => {
                // OpenBracket = "[" => ActionFn(18);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action18::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            31 => {
                __reduce31(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                // __Filter = Filter => ActionFn(2);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(budget, input, __sym0);
                return Some(Ok(__nt));
            }
            44 => {
                __reduce44(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {__action}")
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrExp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompValue, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompareOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Filter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PatchPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValFilter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValuePath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce3<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // AttrPath = AttrNameTok => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce4<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CloseBracket = "]" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "false" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "null" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "eq" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ne" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "co" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce14<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "sw" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce15<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ew" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce16<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "gt" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce17<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "lt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce18<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ge" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce19<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "le" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce20<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter, "or", Filter1 => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    fn __reduce21<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter1 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce22<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter0 = FilterAtom => ActionFn(7);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce23<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter1, "and", Filter0 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce24<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter0 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    fn __reduce25<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = AttrExp => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce26<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = ValuePath => ActionFn(13);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce27<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = "not", Open, Filter, Close => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    fn __reduce28<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = Open, Filter, Close => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    fn __reduce31<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce32<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(48);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action48::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 12)
    }
    fn __reduce33<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket, ".", AttrNameTok => ActionFn(49);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action49::<>(budget, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 12)
    }
    fn __reduce34<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter, "or", ValFilter1 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter1 => ActionFn(29);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    fn __reduce36<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter0 = ValFilterAtom => ActionFn(25);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce37<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter1, "and", ValFilter0 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 15)
    }
    fn __reduce38<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter0 => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    fn __reduce39<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = AttrExp => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 16)
    }
    fn __reduce40<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = "not", Open, ValFilter, Close => ActionFn(31);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action31::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 16)
    }
    fn __reduce41<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = Open, ValFilter, Close => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 16)
    }
    fn __reduce42<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValuePath = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 17)
    }
    fn __reduce44<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter0 = Filter0 => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    fn __reduce45<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter1 = Filter1 => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    fn __reduce46<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Path = Path => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce47<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter = ValFilter => ActionFn(5);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 22)
    }
    fn __reduce48<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter0 = ValFilter0 => ActionFn(3);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 23)
    }
    fn __reduce49<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter1 = ValFilter1 => ActionFn(4);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 24)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Filter::FilterParser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__Filter0 {

    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(AttrExp),
        Variant2(AttrPath),
        Variant3(()),
        Variant4(CompValue),
        Variant5(CompareOp),
        Variant6(Filter),
        Variant7(PatchPath),
        Variant8(ValFilter),
        Variant9(ValuePath),
    }
    const __ACTION: &[i8] = &[
        // State 0
        26, 27, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 29, 30, 31, 32, 33, 34, 35, 36, 0, 37, 38, 0, 0, 0, 28, 0, 0, 0, 0,
        // State 2
        26, 27, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 46, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 43, 44,
        // State 5
        26, 27, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 7
        26, 27, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 29, 30, 31, 32, 33, 34, 35, 36, 0, 37, 38, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        26, 27, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        26, 27, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        26, 27, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 16
        26, 27, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        26, 27, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        26, 27, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0,
        // State 24
        -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3, -3, 0, 0, 0, -3, 0, 0, 0, 0,
        // State 27
        -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14,
        // State 29
        0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12,
        // State 30
        0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 31
        0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19,
        // State 32
        0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 33
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20,
        // State 34
        0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 35
        0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13,
        // State 36
        0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0,
        // State 37
        0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 38
        0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, -2, 0, 0, -2, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, -8, 0, 0, -8, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, -9, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, -10, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, -11, 0, 0, -11, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, -40, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, -39, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, -37, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, -42, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, -35, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, -38, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
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
        -26,
        // State 21
        -45,
        // State 22
        -23,
        // State 23
        -27,
        // State 24
        0,
        // State 25
        0,
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
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        -1,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -2,
        // State 41
        -7,
        // State 42
        -8,
        // State 43
        -9,
        // State 44
        -10,
        // State 45
        -11,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -29,
        // State 51
        -5,
        // State 52
        -43,
        // State 53
        -6,
        // State 54
        0,
        // State 55
        0,
        // State 56
        -28,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                5 | 9 | 16..=18 => 46,
                _ => 20,
            },
            1 => match state {
                5 | 9 | 16..=18 => 8,
                _ => 1,
            },
            2 => match state {
                14 => 56,
                15 => 57,
                19 => 60,
                _ => 50,
            },
            3 => 52,
            4 => 40,
            5 => 4,
            6 => match state {
                7 => 14,
                _ => 6,
            },
            7 => match state {
                0 => 21,
                13 => 55,
                _ => 38,
            },
            8 => match state {
                12 => 54,
                _ => 39,
            },
            9 => 22,
            10 => match state {
                3 => 7,
                5 | 9 | 16..=18 => 9,
                11 => 18,
                _ => 2,
            },
            11 => 5,
            13 => match state {
                9 => 15,
                18 => 19,
                _ => 10,
            },
            14 => match state {
                17 => 59,
                _ => 47,
            },
            15 => match state {
                16 => 58,
                _ => 48,
            },
            16 => 49,
            17 => 23,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"AttrNameTok"###,
        r###"AttrPathTok"###,
        r###"StringLit"###,
        r###""not""###,
        r###"NumberLit"###,
        r###""and""###,
        r###""co""###,
        r###""eq""###,
        r###""ew""###,
        r###""ge""###,
        r###""gt""###,
        r###""le""###,
        r###""lt""###,
        r###""ne""###,
        r###""or""###,
        r###""pr""###,
        r###""sw""###,
        r###""(""###,
        r###"")""###,
        r###"".""###,
        r###""[""###,
        r###""]""###,
        r###""false""###,
        r###""null""###,
        r###""true""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'b,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'b>
    where 
    {
        budget: &'b ParseBudget,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'b ())>,
    }
    impl<'input, 'b> __state_machine::ParserDefinition for __StateMachine<'input, 'b>
    where 
    {
        type Location = usize;
        type Error = FilterActionError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Filter;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
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
                self.budget,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'b,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
            Token(8, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(10, _) if true => Some(10),
            Token(11, _) if true => Some(11),
            Token(12, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(14, _) if true => Some(14),
            Token(15, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            Token(17, _) if true => Some(17),
            Token(18, _) if true => Some(18),
            Token(19, _) if true => Some(19),
            Token(20, _) if true => Some(20),
            Token(21, _) if true => Some(21),
            Token(22, _) if true => Some(22),
            Token(23, _) if true => Some(23),
            Token(24, _) if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'b,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'b,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'b>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => __state_machine::SimulatedReduce::Accept,
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct Filter0Parser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for Filter0Parser { fn default() -> Self { Self::new() } }
    impl Filter0Parser {
        pub fn new() -> Filter0Parser {
            let __builder = super::__intern_token::new_builder();
            Filter0Parser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'b,
        >(
            &self,
            budget: &'b ParseBudget,
            input: &'input str,
        ) -> Result<Filter, __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    budget,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'b,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<Result<Filter,__lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                // AttrExp = AttrPath, "pr" => ActionFn(21);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action21::<>(budget, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            1 => {
                // AttrExp = AttrPath, CompareOp, CompValue => ActionFn(22);
                assert!(__symbols.len() >= 3);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action22::<>(budget, input, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 0)
            }
            2 => {
                // AttrPath = AttrPathTok => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action23::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            3 => {
                __reduce3(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                // CompValue = NumberLit => ActionFn(45);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action45::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            10 => {
                // CompValue = StringLit => ActionFn(46);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            11 => {
                __reduce11(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                // Open = "(" => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action16::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 10)
            }
            30 => {
                // OpenBracket = "[" => ActionFn(18);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action18::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            31 => {
                __reduce31(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                // __Filter0 = Filter0 => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(budget, input, __sym0);
                return Some(Ok(__nt));
            }
            45 => {
                __reduce45(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {__action}")
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrExp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompValue, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompareOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Filter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PatchPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValFilter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValuePath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce3<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // AttrPath = AttrNameTok => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce4<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CloseBracket = "]" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "false" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "null" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "eq" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ne" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "co" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce14<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "sw" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce15<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ew" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce16<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "gt" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce17<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "lt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce18<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ge" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce19<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "le" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce20<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter, "or", Filter1 => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    fn __reduce21<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter1 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce22<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter0 = FilterAtom => ActionFn(7);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce23<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter1, "and", Filter0 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce24<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter0 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    fn __reduce25<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = AttrExp => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce26<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = ValuePath => ActionFn(13);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce27<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = "not", Open, Filter, Close => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    fn __reduce28<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = Open, Filter, Close => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    fn __reduce31<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce32<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(48);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action48::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 12)
    }
    fn __reduce33<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket, ".", AttrNameTok => ActionFn(49);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action49::<>(budget, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 12)
    }
    fn __reduce34<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter, "or", ValFilter1 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter1 => ActionFn(29);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    fn __reduce36<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter0 = ValFilterAtom => ActionFn(25);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce37<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter1, "and", ValFilter0 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 15)
    }
    fn __reduce38<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter0 => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    fn __reduce39<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = AttrExp => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 16)
    }
    fn __reduce40<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = "not", Open, ValFilter, Close => ActionFn(31);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action31::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 16)
    }
    fn __reduce41<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = Open, ValFilter, Close => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 16)
    }
    fn __reduce42<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValuePath = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 17)
    }
    fn __reduce43<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter = Filter => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    fn __reduce45<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter1 = Filter1 => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    fn __reduce46<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Path = Path => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce47<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter = ValFilter => ActionFn(5);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 22)
    }
    fn __reduce48<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter0 = ValFilter0 => ActionFn(3);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 23)
    }
    fn __reduce49<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter1 = ValFilter1 => ActionFn(4);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 24)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Filter0::Filter0Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__Filter1 {

    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(AttrExp),
        Variant2(AttrPath),
        Variant3(()),
        Variant4(CompValue),
        Variant5(CompareOp),
        Variant6(Filter),
        Variant7(PatchPath),
        Variant8(ValFilter),
        Variant9(ValuePath),
    }
    const __ACTION: &[i8] = &[
        // State 0
        27, 28, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 34, 35, 36, 37, 0, 38, 39, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 2
        27, 28, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 46, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 43, 44,
        // State 5
        27, 28, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        27, 28, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0,
        // State 8
        27, 28, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 30, 31, 32, 33, 34, 35, 36, 37, 0, 38, 39, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        27, 28, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        27, 28, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0,
        // State 16
        27, 28, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        27, 28, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        27, 28, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0,
        // State 25
        -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3, -3, 0, 0, 0, -3, 0, 0, 0, 0,
        // State 28
        -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14,
        // State 30
        0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12,
        // State 31
        0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 32
        0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19,
        // State 33
        0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 34
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20,
        // State 35
        0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 36
        0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13,
        // State 37
        0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0,
        // State 38
        0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 39
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, -2, 0, 0, -2, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, -8, 0, 0, -8, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, -9, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, -10, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, -11, 0, 0, -11, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, -40, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, -39, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, -37, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, -42, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, -35, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, -38, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
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
        -26,
        // State 21
        -25,
        // State 22
        -46,
        // State 23
        -23,
        // State 24
        -27,
        // State 25
        0,
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
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -1,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -2,
        // State 41
        -7,
        // State 42
        -8,
        // State 43
        -9,
        // State 44
        -10,
        // State 45
        -11,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -24,
        // State 51
        -29,
        // State 52
        -5,
        // State 53
        -43,
        // State 54
        -6,
        // State 55
        0,
        // State 56
        -28,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                5 | 10 | 16..=18 => 46,
                _ => 20,
            },
            1 => match state {
                5 | 10 | 16..=18 => 9,
                _ => 1,
            },
            2 => match state {
                14 => 56,
                15 => 57,
                19 => 60,
                _ => 51,
            },
            3 => 53,
            4 => 40,
            5 => 4,
            6 => match state {
                8 => 14,
                _ => 7,
            },
            7 => match state {
                6 => 50,
                _ => 21,
            },
            8 => match state {
                0 => 22,
                13 => 55,
                _ => 39,
            },
            9 => 23,
            10 => match state {
                3 => 8,
                5 | 10 | 16..=18 => 10,
                12 => 18,
                _ => 2,
            },
            11 => 5,
            13 => match state {
                10 => 15,
                18 => 19,
                _ => 11,
            },
            14 => match state {
                17 => 59,
                _ => 47,
            },
            15 => match state {
                16 => 58,
                _ => 48,
            },
            16 => 49,
            17 => 24,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"AttrNameTok"###,
        r###"AttrPathTok"###,
        r###"StringLit"###,
        r###""not""###,
        r###"NumberLit"###,
        r###""and""###,
        r###""co""###,
        r###""eq""###,
        r###""ew""###,
        r###""ge""###,
        r###""gt""###,
        r###""le""###,
        r###""lt""###,
        r###""ne""###,
        r###""or""###,
        r###""pr""###,
        r###""sw""###,
        r###""(""###,
        r###"")""###,
        r###"".""###,
        r###""[""###,
        r###""]""###,
        r###""false""###,
        r###""null""###,
        r###""true""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'b,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'b>
    where 
    {
        budget: &'b ParseBudget,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'b ())>,
    }
    impl<'input, 'b> __state_machine::ParserDefinition for __StateMachine<'input, 'b>
    where 
    {
        type Location = usize;
        type Error = FilterActionError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Filter;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
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
                self.budget,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'b,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
            Token(8, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(10, _) if true => Some(10),
            Token(11, _) if true => Some(11),
            Token(12, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(14, _) if true => Some(14),
            Token(15, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            Token(17, _) if true => Some(17),
            Token(18, _) if true => Some(18),
            Token(19, _) if true => Some(19),
            Token(20, _) if true => Some(20),
            Token(21, _) if true => Some(21),
            Token(22, _) if true => Some(22),
            Token(23, _) if true => Some(23),
            Token(24, _) if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'b,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'b,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'b>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            45 => __state_machine::SimulatedReduce::Accept,
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct Filter1Parser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for Filter1Parser { fn default() -> Self { Self::new() } }
    impl Filter1Parser {
        pub fn new() -> Filter1Parser {
            let __builder = super::__intern_token::new_builder();
            Filter1Parser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'b,
        >(
            &self,
            budget: &'b ParseBudget,
            input: &'input str,
        ) -> Result<Filter, __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    budget,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'b,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<Result<Filter,__lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                // AttrExp = AttrPath, "pr" => ActionFn(21);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action21::<>(budget, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            1 => {
                // AttrExp = AttrPath, CompareOp, CompValue => ActionFn(22);
                assert!(__symbols.len() >= 3);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action22::<>(budget, input, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 0)
            }
            2 => {
                // AttrPath = AttrPathTok => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action23::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            3 => {
                __reduce3(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                // CompValue = NumberLit => ActionFn(45);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action45::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            10 => {
                // CompValue = StringLit => ActionFn(46);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            11 => {
                __reduce11(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                // Open = "(" => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action16::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 10)
            }
            30 => {
                // OpenBracket = "[" => ActionFn(18);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action18::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            31 => {
                __reduce31(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                // __Filter1 = Filter1 => ActionFn(1);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(budget, input, __sym0);
                return Some(Ok(__nt));
            }
            46 => {
                __reduce46(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {__action}")
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrExp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompValue, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompareOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Filter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PatchPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValFilter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValuePath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce3<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // AttrPath = AttrNameTok => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce4<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CloseBracket = "]" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "false" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "null" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "eq" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ne" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "co" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce14<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "sw" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce15<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ew" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce16<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "gt" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce17<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "lt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce18<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ge" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce19<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "le" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce20<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter, "or", Filter1 => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    fn __reduce21<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter1 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce22<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter0 = FilterAtom => ActionFn(7);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce23<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter1, "and", Filter0 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce24<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter0 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    fn __reduce25<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = AttrExp => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce26<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = ValuePath => ActionFn(13);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce27<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = "not", Open, Filter, Close => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    fn __reduce28<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = Open, Filter, Close => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    fn __reduce31<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce32<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(48);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action48::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 12)
    }
    fn __reduce33<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket, ".", AttrNameTok => ActionFn(49);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action49::<>(budget, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 12)
    }
    fn __reduce34<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter, "or", ValFilter1 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter1 => ActionFn(29);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    fn __reduce36<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter0 = ValFilterAtom => ActionFn(25);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce37<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter1, "and", ValFilter0 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 15)
    }
    fn __reduce38<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter0 => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    fn __reduce39<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = AttrExp => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 16)
    }
    fn __reduce40<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = "not", Open, ValFilter, Close => ActionFn(31);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action31::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 16)
    }
    fn __reduce41<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = Open, ValFilter, Close => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 16)
    }
    fn __reduce42<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValuePath = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 17)
    }
    fn __reduce43<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter = Filter => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    fn __reduce44<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter0 = Filter0 => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    fn __reduce46<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Path = Path => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce47<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter = ValFilter => ActionFn(5);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 22)
    }
    fn __reduce48<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter0 = ValFilter0 => ActionFn(3);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 23)
    }
    fn __reduce49<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter1 = ValFilter1 => ActionFn(4);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 24)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Filter1::Filter1Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__Path {

    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(AttrExp),
        Variant2(AttrPath),
        Variant3(()),
        Variant4(CompValue),
        Variant5(CompareOp),
        Variant6(Filter),
        Variant7(PatchPath),
        Variant8(ValFilter),
        Variant9(ValuePath),
    }
    const __ACTION: &[i8] = &[
        // State 0
        15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0,
        // State 2
        15, 16, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 23, 24, 25, 26, 27, 28, 29, 30, 0, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        15, 16, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 40, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0,
        // State 9
        15, 16, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        15, 16, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        15, 16, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3, -3, 0, 0, 0, -3, 0, 0, 0, 0,
        // State 16
        -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, -40, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, -39, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, -36, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, -37, 0, 0, 0,
        // State 21
        -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14,
        // State 23
        0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12,
        // State 24
        0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 25
        0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19,
        // State 26
        0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 27
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20,
        // State 28
        0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 29
        0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13,
        // State 30
        0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0,
        // State 31
        0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, -2, 0, 0, -2, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, -8, 0, 0, -8, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, -9, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, -10, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, -11, 0, 0, -11, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, -42, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0,
        // State 42
        46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, -35, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, -38, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -32,
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
        -47,
        // State 14
        -4,
        // State 15
        -3,
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
        0,
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
        0,
        // State 32
        -33,
        // State 33
        -6,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -34,
        // State 46
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 17,
            1 => match state {
                0 => 1,
                _ => 3,
            },
            2 => match state {
                12 => 46,
                _ => 40,
            },
            3 => 32,
            4 => 34,
            5 => 7,
            10 => match state {
                6 => 11,
                _ => 4,
            },
            11 => 2,
            12 => 13,
            13 => match state {
                4 => 8,
                11 => 12,
                _ => 5,
            },
            14 => match state {
                10 => 44,
                _ => 18,
            },
            15 => match state {
                9 => 43,
                _ => 19,
            },
            16 => 20,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"AttrNameTok"###,
        r###"AttrPathTok"###,
        r###"StringLit"###,
        r###""not""###,
        r###"NumberLit"###,
        r###""and""###,
        r###""co""###,
        r###""eq""###,
        r###""ew""###,
        r###""ge""###,
        r###""gt""###,
        r###""le""###,
        r###""lt""###,
        r###""ne""###,
        r###""or""###,
        r###""pr""###,
        r###""sw""###,
        r###""(""###,
        r###"")""###,
        r###"".""###,
        r###""[""###,
        r###""]""###,
        r###""false""###,
        r###""null""###,
        r###""true""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'b,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'b>
    where 
    {
        budget: &'b ParseBudget,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'b ())>,
    }
    impl<'input, 'b> __state_machine::ParserDefinition for __StateMachine<'input, 'b>
    where 
    {
        type Location = usize;
        type Error = FilterActionError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = PatchPath;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
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
                self.budget,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'b,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
            Token(8, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(10, _) if true => Some(10),
            Token(11, _) if true => Some(11),
            Token(12, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(14, _) if true => Some(14),
            Token(15, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            Token(17, _) if true => Some(17),
            Token(18, _) if true => Some(18),
            Token(19, _) if true => Some(19),
            Token(20, _) if true => Some(20),
            Token(21, _) if true => Some(21),
            Token(22, _) if true => Some(22),
            Token(23, _) if true => Some(23),
            Token(24, _) if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'b,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'b,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'b>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            46 => __state_machine::SimulatedReduce::Accept,
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct PathParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for PathParser { fn default() -> Self { Self::new() } }
    impl PathParser {
        pub fn new() -> PathParser {
            let __builder = super::__intern_token::new_builder();
            PathParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'b,
        >(
            &self,
            budget: &'b ParseBudget,
            input: &'input str,
        ) -> Result<PatchPath, __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    budget,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'b,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<Result<PatchPath,__lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                // AttrExp = AttrPath, "pr" => ActionFn(21);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action21::<>(budget, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            1 => {
                // AttrExp = AttrPath, CompareOp, CompValue => ActionFn(22);
                assert!(__symbols.len() >= 3);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action22::<>(budget, input, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 0)
            }
            2 => {
                // AttrPath = AttrPathTok => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action23::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            3 => {
                __reduce3(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                // CompValue = NumberLit => ActionFn(45);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action45::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            10 => {
                // CompValue = StringLit => ActionFn(46);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            11 => {
                __reduce11(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                // Open = "(" => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action16::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 10)
            }
            30 => {
                // OpenBracket = "[" => ActionFn(18);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action18::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            31 => {
                __reduce31(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                // __Path = Path => ActionFn(6);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(budget, input, __sym0);
                return Some(Ok(__nt));
            }
            47 => {
                __reduce47(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {__action}")
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrExp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompValue, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompareOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Filter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PatchPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValFilter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValuePath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce3<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // AttrPath = AttrNameTok => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce4<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CloseBracket = "]" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "false" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "null" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "eq" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ne" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "co" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce14<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "sw" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce15<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ew" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce16<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "gt" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce17<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "lt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce18<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ge" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce19<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "le" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce20<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter, "or", Filter1 => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    fn __reduce21<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter1 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce22<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter0 = FilterAtom => ActionFn(7);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce23<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter1, "and", Filter0 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce24<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter0 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    fn __reduce25<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = AttrExp => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce26<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = ValuePath => ActionFn(13);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce27<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = "not", Open, Filter, Close => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    fn __reduce28<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = Open, Filter, Close => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    fn __reduce31<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce32<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(48);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action48::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 12)
    }
    fn __reduce33<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket, ".", AttrNameTok => ActionFn(49);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action49::<>(budget, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 12)
    }
    fn __reduce34<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter, "or", ValFilter1 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter1 => ActionFn(29);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    fn __reduce36<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter0 = ValFilterAtom => ActionFn(25);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce37<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter1, "and", ValFilter0 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 15)
    }
    fn __reduce38<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter0 => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    fn __reduce39<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = AttrExp => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 16)
    }
    fn __reduce40<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = "not", Open, ValFilter, Close => ActionFn(31);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action31::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 16)
    }
    fn __reduce41<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = Open, ValFilter, Close => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 16)
    }
    fn __reduce42<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValuePath = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 17)
    }
    fn __reduce43<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter = Filter => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    fn __reduce44<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter0 = Filter0 => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    fn __reduce45<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter1 = Filter1 => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    fn __reduce47<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter = ValFilter => ActionFn(5);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 22)
    }
    fn __reduce48<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter0 = ValFilter0 => ActionFn(3);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 23)
    }
    fn __reduce49<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter1 = ValFilter1 => ActionFn(4);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 24)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Path::PathParser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__ValFilter {

    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(AttrExp),
        Variant2(AttrPath),
        Variant3(()),
        Variant4(CompValue),
        Variant5(CompareOp),
        Variant6(Filter),
        Variant7(PatchPath),
        Variant8(ValFilter),
        Variant9(ValuePath),
    }
    const __ACTION: &[i8] = &[
        // State 0
        17, 18, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25, 26, 0, 27, 28, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        17, 18, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 34, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0,
        // State 6
        17, 18, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        17, 18, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        17, 18, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0,
        // State 15
        -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3, -3, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14,
        // State 19
        0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12,
        // State 20
        0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 21
        0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19,
        // State 22
        0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 23
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20,
        // State 24
        0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 25
        0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13,
        // State 26
        0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 28
        0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
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
        -40,
        // State 11
        -48,
        // State 12
        -39,
        // State 13
        -36,
        // State 14
        -37,
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
        0,
        // State 26
        -1,
        // State 27
        0,
        // State 28
        -2,
        // State 29
        -7,
        // State 30
        -8,
        // State 31
        -9,
        // State 32
        -10,
        // State 33
        -11,
        // State 34
        -42,
        // State 35
        -5,
        // State 36
        -35,
        // State 37
        -38,
        // State 38
        -41,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 10,
            1 => 1,
            2 => match state {
                9 => 38,
                _ => 34,
            },
            4 => 28,
            5 => 4,
            10 => match state {
                3 => 8,
                _ => 2,
            },
            13 => match state {
                8 => 9,
                0 => 11,
                _ => 5,
            },
            14 => match state {
                7 => 37,
                _ => 12,
            },
            15 => match state {
                6 => 36,
                _ => 13,
            },
            16 => 14,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"AttrNameTok"###,
        r###"AttrPathTok"###,
        r###"StringLit"###,
        r###""not""###,
        r###"NumberLit"###,
        r###""and""###,
        r###""co""###,
        r###""eq""###,
        r###""ew""###,
        r###""ge""###,
        r###""gt""###,
        r###""le""###,
        r###""lt""###,
        r###""ne""###,
        r###""or""###,
        r###""pr""###,
        r###""sw""###,
        r###""(""###,
        r###"")""###,
        r###"".""###,
        r###""[""###,
        r###""]""###,
        r###""false""###,
        r###""null""###,
        r###""true""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'b,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'b>
    where 
    {
        budget: &'b ParseBudget,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'b ())>,
    }
    impl<'input, 'b> __state_machine::ParserDefinition for __StateMachine<'input, 'b>
    where 
    {
        type Location = usize;
        type Error = FilterActionError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ValFilter;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
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
                self.budget,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'b,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
            Token(8, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(10, _) if true => Some(10),
            Token(11, _) if true => Some(11),
            Token(12, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(14, _) if true => Some(14),
            Token(15, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            Token(17, _) if true => Some(17),
            Token(18, _) if true => Some(18),
            Token(19, _) if true => Some(19),
            Token(20, _) if true => Some(20),
            Token(21, _) if true => Some(21),
            Token(22, _) if true => Some(22),
            Token(23, _) if true => Some(23),
            Token(24, _) if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'b,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'b,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'b>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            47 => __state_machine::SimulatedReduce::Accept,
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct ValFilterParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for ValFilterParser { fn default() -> Self { Self::new() } }
    impl ValFilterParser {
        pub fn new() -> ValFilterParser {
            let __builder = super::__intern_token::new_builder();
            ValFilterParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'b,
        >(
            &self,
            budget: &'b ParseBudget,
            input: &'input str,
        ) -> Result<ValFilter, __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    budget,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'b,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<Result<ValFilter,__lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                // AttrExp = AttrPath, "pr" => ActionFn(21);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action21::<>(budget, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            1 => {
                // AttrExp = AttrPath, CompareOp, CompValue => ActionFn(22);
                assert!(__symbols.len() >= 3);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action22::<>(budget, input, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 0)
            }
            2 => {
                // AttrPath = AttrPathTok => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action23::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            3 => {
                __reduce3(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                // CompValue = NumberLit => ActionFn(45);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action45::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            10 => {
                // CompValue = StringLit => ActionFn(46);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            11 => {
                __reduce11(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                // Open = "(" => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action16::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 10)
            }
            30 => {
                // OpenBracket = "[" => ActionFn(18);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action18::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            31 => {
                __reduce31(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                // __ValFilter = ValFilter => ActionFn(5);
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(budget, input, __sym0);
                return Some(Ok(__nt));
            }
            48 => {
                __reduce48(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {__action}")
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrExp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompValue, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompareOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Filter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PatchPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValFilter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValuePath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce3<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // AttrPath = AttrNameTok => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce4<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CloseBracket = "]" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "false" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "null" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "eq" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ne" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "co" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce14<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "sw" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce15<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ew" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce16<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "gt" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce17<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "lt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce18<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ge" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce19<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "le" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce20<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter, "or", Filter1 => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    fn __reduce21<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter1 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce22<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter0 = FilterAtom => ActionFn(7);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce23<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter1, "and", Filter0 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce24<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter0 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    fn __reduce25<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = AttrExp => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce26<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = ValuePath => ActionFn(13);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce27<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = "not", Open, Filter, Close => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    fn __reduce28<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = Open, Filter, Close => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    fn __reduce31<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce32<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(48);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action48::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 12)
    }
    fn __reduce33<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket, ".", AttrNameTok => ActionFn(49);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action49::<>(budget, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 12)
    }
    fn __reduce34<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter, "or", ValFilter1 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter1 => ActionFn(29);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    fn __reduce36<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter0 = ValFilterAtom => ActionFn(25);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce37<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter1, "and", ValFilter0 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 15)
    }
    fn __reduce38<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter0 => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    fn __reduce39<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = AttrExp => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 16)
    }
    fn __reduce40<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = "not", Open, ValFilter, Close => ActionFn(31);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action31::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 16)
    }
    fn __reduce41<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = Open, ValFilter, Close => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 16)
    }
    fn __reduce42<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValuePath = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 17)
    }
    fn __reduce43<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter = Filter => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    fn __reduce44<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter0 = Filter0 => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    fn __reduce45<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter1 = Filter1 => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    fn __reduce46<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Path = Path => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce48<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter0 = ValFilter0 => ActionFn(3);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 23)
    }
    fn __reduce49<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter1 = ValFilter1 => ActionFn(4);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 24)
    }
}
#[allow(unused_imports)]
pub use self::__parse__ValFilter::ValFilterParser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__ValFilter0 {

    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(AttrExp),
        Variant2(AttrPath),
        Variant3(()),
        Variant4(CompValue),
        Variant5(CompareOp),
        Variant6(Filter),
        Variant7(PatchPath),
        Variant8(ValFilter),
        Variant9(ValuePath),
    }
    const __ACTION: &[i8] = &[
        // State 0
        15, 16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 17, 18, 19, 20, 21, 22, 23, 24, 0, 25, 26, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        15, 16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 34, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0,
        // State 6
        15, 16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        15, 16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        15, 16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0,
        // State 13
        -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3, -3, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14,
        // State 17
        0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12,
        // State 18
        0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 19
        0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19,
        // State 20
        0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 21
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20,
        // State 22
        0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 23
        0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13,
        // State 24
        0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 26
        0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
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
        -40,
        // State 11
        -49,
        // State 12
        -37,
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
        -1,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -2,
        // State 29
        -7,
        // State 30
        -8,
        // State 31
        -9,
        // State 32
        -10,
        // State 33
        -11,
        // State 34
        -42,
        // State 35
        -5,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -41,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 10,
            1 => 1,
            2 => match state {
                9 => 38,
                _ => 34,
            },
            4 => 28,
            5 => 4,
            10 => match state {
                3 => 6,
                _ => 2,
            },
            13 => match state {
                6 => 9,
                _ => 5,
            },
            14 => match state {
                0 => 11,
                8 => 37,
                _ => 26,
            },
            15 => match state {
                7 => 36,
                _ => 27,
            },
            16 => 12,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"AttrNameTok"###,
        r###"AttrPathTok"###,
        r###"StringLit"###,
        r###""not""###,
        r###"NumberLit"###,
        r###""and""###,
        r###""co""###,
        r###""eq""###,
        r###""ew""###,
        r###""ge""###,
        r###""gt""###,
        r###""le""###,
        r###""lt""###,
        r###""ne""###,
        r###""or""###,
        r###""pr""###,
        r###""sw""###,
        r###""(""###,
        r###"")""###,
        r###"".""###,
        r###""[""###,
        r###""]""###,
        r###""false""###,
        r###""null""###,
        r###""true""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'b,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'b>
    where 
    {
        budget: &'b ParseBudget,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'b ())>,
    }
    impl<'input, 'b> __state_machine::ParserDefinition for __StateMachine<'input, 'b>
    where 
    {
        type Location = usize;
        type Error = FilterActionError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ValFilter;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
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
                self.budget,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'b,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
            Token(8, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(10, _) if true => Some(10),
            Token(11, _) if true => Some(11),
            Token(12, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(14, _) if true => Some(14),
            Token(15, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            Token(17, _) if true => Some(17),
            Token(18, _) if true => Some(18),
            Token(19, _) if true => Some(19),
            Token(20, _) if true => Some(20),
            Token(21, _) if true => Some(21),
            Token(22, _) if true => Some(22),
            Token(23, _) if true => Some(23),
            Token(24, _) if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'b,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'b,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'b>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            48 => __state_machine::SimulatedReduce::Accept,
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct ValFilter0Parser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for ValFilter0Parser { fn default() -> Self { Self::new() } }
    impl ValFilter0Parser {
        pub fn new() -> ValFilter0Parser {
            let __builder = super::__intern_token::new_builder();
            ValFilter0Parser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'b,
        >(
            &self,
            budget: &'b ParseBudget,
            input: &'input str,
        ) -> Result<ValFilter, __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    budget,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'b,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<Result<ValFilter,__lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                // AttrExp = AttrPath, "pr" => ActionFn(21);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action21::<>(budget, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            1 => {
                // AttrExp = AttrPath, CompareOp, CompValue => ActionFn(22);
                assert!(__symbols.len() >= 3);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action22::<>(budget, input, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 0)
            }
            2 => {
                // AttrPath = AttrPathTok => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action23::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            3 => {
                __reduce3(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                // CompValue = NumberLit => ActionFn(45);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action45::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            10 => {
                // CompValue = StringLit => ActionFn(46);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            11 => {
                __reduce11(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                // Open = "(" => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action16::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 10)
            }
            30 => {
                // OpenBracket = "[" => ActionFn(18);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action18::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            31 => {
                __reduce31(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                // __ValFilter0 = ValFilter0 => ActionFn(3);
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(budget, input, __sym0);
                return Some(Ok(__nt));
            }
            49 => {
                __reduce49(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {__action}")
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrExp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompValue, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompareOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Filter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PatchPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValFilter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValuePath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce3<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // AttrPath = AttrNameTok => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce4<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CloseBracket = "]" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "false" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "null" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "eq" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ne" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "co" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce14<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "sw" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce15<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ew" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce16<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "gt" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce17<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "lt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce18<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ge" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce19<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "le" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce20<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter, "or", Filter1 => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    fn __reduce21<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter1 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce22<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter0 = FilterAtom => ActionFn(7);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce23<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter1, "and", Filter0 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce24<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter0 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    fn __reduce25<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = AttrExp => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce26<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = ValuePath => ActionFn(13);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce27<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = "not", Open, Filter, Close => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    fn __reduce28<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = Open, Filter, Close => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    fn __reduce31<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce32<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(48);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action48::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 12)
    }
    fn __reduce33<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket, ".", AttrNameTok => ActionFn(49);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action49::<>(budget, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 12)
    }
    fn __reduce34<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter, "or", ValFilter1 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter1 => ActionFn(29);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    fn __reduce36<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter0 = ValFilterAtom => ActionFn(25);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce37<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter1, "and", ValFilter0 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 15)
    }
    fn __reduce38<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter0 => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    fn __reduce39<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = AttrExp => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 16)
    }
    fn __reduce40<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = "not", Open, ValFilter, Close => ActionFn(31);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action31::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 16)
    }
    fn __reduce41<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = Open, ValFilter, Close => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 16)
    }
    fn __reduce42<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValuePath = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 17)
    }
    fn __reduce43<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter = Filter => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    fn __reduce44<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter0 = Filter0 => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    fn __reduce45<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter1 = Filter1 => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    fn __reduce46<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Path = Path => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce47<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter = ValFilter => ActionFn(5);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 22)
    }
    fn __reduce49<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter1 = ValFilter1 => ActionFn(4);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 24)
    }
}
#[allow(unused_imports)]
pub use self::__parse__ValFilter0::ValFilter0Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__ValFilter1 {

    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(AttrExp),
        Variant2(AttrPath),
        Variant3(()),
        Variant4(CompValue),
        Variant5(CompareOp),
        Variant6(Filter),
        Variant7(PatchPath),
        Variant8(ValFilter),
        Variant9(ValuePath),
    }
    const __ACTION: &[i8] = &[
        // State 0
        16, 17, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 18, 19, 20, 21, 22, 23, 24, 25, 0, 26, 27, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        16, 17, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 34, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0,
        // State 6
        16, 17, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        16, 17, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        16, 17, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0,
        // State 14
        -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3, -3, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14,
        // State 18
        0, 0, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12,
        // State 19
        0, 0, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 20
        0, 0, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19,
        // State 21
        0, 0, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 22
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20,
        // State 23
        0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 24
        0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13,
        // State 25
        0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 27
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 25 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
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
        -40,
        // State 11
        -39,
        // State 12
        -50,
        // State 13
        -37,
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
        -1,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -2,
        // State 29
        -7,
        // State 30
        -8,
        // State 31
        -9,
        // State 32
        -10,
        // State 33
        -11,
        // State 34
        -42,
        // State 35
        -5,
        // State 36
        -38,
        // State 37
        0,
        // State 38
        -41,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 10,
            1 => 1,
            2 => match state {
                9 => 38,
                _ => 34,
            },
            4 => 28,
            5 => 4,
            10 => match state {
                3 => 7,
                _ => 2,
            },
            13 => match state {
                7 => 9,
                _ => 5,
            },
            14 => match state {
                6 => 36,
                _ => 11,
            },
            15 => match state {
                0 => 12,
                8 => 37,
                _ => 27,
            },
            16 => 13,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"AttrNameTok"###,
        r###"AttrPathTok"###,
        r###"StringLit"###,
        r###""not""###,
        r###"NumberLit"###,
        r###""and""###,
        r###""co""###,
        r###""eq""###,
        r###""ew""###,
        r###""ge""###,
        r###""gt""###,
        r###""le""###,
        r###""lt""###,
        r###""ne""###,
        r###""or""###,
        r###""pr""###,
        r###""sw""###,
        r###""(""###,
        r###"")""###,
        r###"".""###,
        r###""[""###,
        r###""]""###,
        r###""false""###,
        r###""null""###,
        r###""true""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
        'b,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&(), &())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input, 'b>
    where 
    {
        budget: &'b ParseBudget,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'b ())>,
    }
    impl<'input, 'b> __state_machine::ParserDefinition for __StateMachine<'input, 'b>
    where 
    {
        type Location = usize;
        type Error = FilterActionError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ValFilter;
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
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 25 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&(), &())>)
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
                self.budget,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'b,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
            Token(8, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(10, _) if true => Some(10),
            Token(11, _) if true => Some(11),
            Token(12, _) if true => Some(12),
            Token(13, _) if true => Some(13),
            Token(14, _) if true => Some(14),
            Token(15, _) if true => Some(15),
            Token(16, _) if true => Some(16),
            Token(17, _) if true => Some(17),
            Token(18, _) if true => Some(18),
            Token(19, _) if true => Some(19),
            Token(20, _) if true => Some(20),
            Token(21, _) if true => Some(21),
            Token(22, _) if true => Some(22),
            Token(23, _) if true => Some(23),
            Token(24, _) if true => Some(24),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'b,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'b,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'b>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            49 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct ValFilter1Parser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for ValFilter1Parser { fn default() -> Self { Self::new() } }
    impl ValFilter1Parser {
        pub fn new() -> ValFilter1Parser {
            let __builder = super::__intern_token::new_builder();
            ValFilter1Parser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'b,
        >(
            &self,
            budget: &'b ParseBudget,
            input: &'input str,
        ) -> Result<ValFilter, __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    budget,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'b,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> Option<Result<ValFilter,__lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                // AttrExp = AttrPath, "pr" => ActionFn(21);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action21::<>(budget, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            1 => {
                // AttrExp = AttrPath, CompareOp, CompValue => ActionFn(22);
                assert!(__symbols.len() >= 3);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant5(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action22::<>(budget, input, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 0)
            }
            2 => {
                // AttrPath = AttrPathTok => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action23::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            3 => {
                __reduce3(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                // CompValue = NumberLit => ActionFn(45);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action45::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            10 => {
                // CompValue = StringLit => ActionFn(46);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            11 => {
                __reduce11(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                // Open = "(" => ActionFn(16);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action16::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 10)
            }
            30 => {
                // OpenBracket = "[" => ActionFn(18);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action18::<>(budget, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            31 => {
                __reduce31(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(budget, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                // __ValFilter1 = ValFilter1 => ActionFn(4);
                let __sym0 = __pop_Variant8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(budget, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {__action}")
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrExp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AttrPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompValue, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CompareOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Filter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PatchPath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValFilter, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ValuePath, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce3<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // AttrPath = AttrNameTok => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce4<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CloseBracket = "]" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    fn __reduce6<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "false" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce7<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "null" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompValue = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "eq" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ne" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "co" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce14<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "sw" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce15<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ew" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce16<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "gt" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce17<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "lt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce18<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "ge" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce19<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // CompareOp = "le" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce20<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter, "or", Filter1 => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    fn __reduce21<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter = Filter1 => ActionFn(11);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce22<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter0 = FilterAtom => ActionFn(7);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce23<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter1, "and", Filter0 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce24<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Filter1 = Filter0 => ActionFn(9);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    fn __reduce25<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = AttrExp => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce26<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = ValuePath => ActionFn(13);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    fn __reduce27<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = "not", Open, Filter, Close => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    fn __reduce28<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // FilterAtom = Open, Filter, Close => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    fn __reduce31<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce32<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(48);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action48::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 12)
    }
    fn __reduce33<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // Path = AttrPath, OpenBracket, ValFilter, CloseBracket, ".", AttrNameTok => ActionFn(49);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action49::<>(budget, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (6, 12)
    }
    fn __reduce34<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter, "or", ValFilter1 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter = ValFilter1 => ActionFn(29);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    fn __reduce36<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter0 = ValFilterAtom => ActionFn(25);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    fn __reduce37<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter1, "and", ValFilter0 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 15)
    }
    fn __reduce38<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilter1 = ValFilter0 => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    fn __reduce39<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = AttrExp => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 16)
    }
    fn __reduce40<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = "not", Open, ValFilter, Close => ActionFn(31);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action31::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 16)
    }
    fn __reduce41<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValFilterAtom = Open, ValFilter, Close => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(budget, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 16)
    }
    fn __reduce42<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // ValuePath = AttrPath, OpenBracket, ValFilter, CloseBracket => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(budget, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 17)
    }
    fn __reduce43<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter = Filter => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    fn __reduce44<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter0 = Filter0 => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    fn __reduce45<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Filter1 = Filter1 => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 20)
    }
    fn __reduce46<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __Path = Path => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce47<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter = ValFilter => ActionFn(5);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 22)
    }
    fn __reduce48<
        'input,
        'b,
    >(
        budget: &'b ParseBudget,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'b ())>,
    ) -> (usize, usize)
    {
        // __ValFilter0 = ValFilter0 => ActionFn(3);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(budget, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 23)
    }
}
#[allow(unused_imports)]
pub use self::__parse__ValFilter1::ValFilter1Parser;
#[rustfmt::skip]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::filter::{
    AttrExp, AttrPath, CompareOp, CompValue, Filter, FilterActionError, ParseBudget,
    PatchPath, PatchValuePath, ValFilter, ValuePath, parse_attr_path,
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("(?:[A-Za-z][\\-0-9A-Z_a-z]*)", false),
            ("(?:[A-Za-z][\\-0-9A-Z_a-z]*[\\.:][\\-\\.0-:A-Z_a-z]*)", false),
            ("(?:\"((?:[\0-!\\#-\\[\\]-\u{10ffff}]|(?:\\\\[\0-\t\u{b}-\u{10ffff}])))*\")", false),
            ("(?:[Nn][Oo][Tt][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:\\-?(?:0|(?:[1-9][0-9]*))(?:\\.[0-9]+)?(?:[Ee][\\+\\-]?[0-9]+)?)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Aa][Nn][Dd][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Cc][Oo][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Ee][Qq][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Ee][Ww][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Gg][Ee][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Gg][Tt][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Ll][Ee][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Ll][Tt][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Nn][Ee][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Oo][Rr][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Pp][Rr])", false),
            ("(?:[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+[Ssſ][Ww][\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]+)", false),
            ("\\(", false),
            ("\\)", false),
            ("\\.", false),
            ("\\[", false),
            ("\\]", false),
            ("(?:false)", false),
            ("(?:null)", false),
            ("(?:true)", false),
            (r"\s+", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, Filter, usize),
) -> Filter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, Filter, usize),
) -> Filter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, Filter, usize),
) -> Filter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, ValFilter, usize),
) -> ValFilter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, ValFilter, usize),
) -> ValFilter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, ValFilter, usize),
) -> ValFilter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, PatchPath, usize),
) -> PatchPath
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, Filter, usize),
) -> Filter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, l, _): (usize, Filter, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Filter, usize),
) -> Filter
{
    Filter::and(l, r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action9<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, Filter, usize),
) -> Filter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action10<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, l, _): (usize, Filter, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Filter, usize),
) -> Filter
{
    Filter::or(l, r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action11<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, Filter, usize),
) -> Filter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action12<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, e, _): (usize, AttrExp, usize),
) -> Filter
{
    Filter::Attr(e)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action13<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, v, _): (usize, ValuePath, usize),
) -> Filter
{
    Filter::ValuePath(v)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action14<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, (), usize),
    (_, f, _): (usize, Filter, usize),
    (_, _, _): (usize, (), usize),
) -> Filter
{
    Filter::Not(Box::new(f))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action15<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, f, _): (usize, Filter, usize),
    (_, _, _): (usize, (), usize),
) -> Filter
{
    f
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action16<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<(),__lalrpop_util::ParseError<usize,Token<'input>,FilterActionError>>
{
    Ok(budget.enter()?)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action17<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
)
{
    budget.leave()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action18<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<(),__lalrpop_util::ParseError<usize,Token<'input>,FilterActionError>>
{
    Ok(budget.enter()?)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action19<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
)
{
    budget.leave()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action20<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, a, _): (usize, AttrPath, usize),
    (_, _, _): (usize, (), usize),
    (_, vf, _): (usize, ValFilter, usize),
    (_, _, _): (usize, (), usize),
) -> ValuePath
{
    ValuePath {
        attr: a,
        filter: Box::new(vf),
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action21<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, a, _): (usize, AttrPath, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Result<AttrExp,__lalrpop_util::ParseError<usize,Token<'input>,FilterActionError>>
{
    { budget.term()?; Ok(AttrExp::Present(a)) }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action22<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, a, _): (usize, AttrPath, usize),
    (_, op, _): (usize, CompareOp, usize),
    (_, v, _): (usize, CompValue, usize),
) -> Result<AttrExp,__lalrpop_util::ParseError<usize,Token<'input>,FilterActionError>>
{
    { budget.term()?; Ok(AttrExp::Comparison(a, op, v)) }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action23<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Result<AttrPath,__lalrpop_util::ParseError<usize,Token<'input>,FilterActionError>>
{
    Ok(parse_attr_path(s)?)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action24<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> AttrPath
{
    AttrPath { uri: None, name: s.to_string(), sub_attr: None }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action25<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, ValFilter, usize),
) -> ValFilter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action26<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, l, _): (usize, ValFilter, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, ValFilter, usize),
) -> ValFilter
{
    ValFilter::and(l, r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action27<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, ValFilter, usize),
) -> ValFilter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action28<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, l, _): (usize, ValFilter, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, ValFilter, usize),
) -> ValFilter
{
    ValFilter::or(l, r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action29<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, ValFilter, usize),
) -> ValFilter
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action30<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, e, _): (usize, AttrExp, usize),
) -> ValFilter
{
    ValFilter::Attr(e)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action31<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, (), usize),
    (_, f, _): (usize, ValFilter, usize),
    (_, _, _): (usize, (), usize),
) -> ValFilter
{
    ValFilter::Not(Box::new(f))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action32<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, f, _): (usize, ValFilter, usize),
    (_, _, _): (usize, (), usize),
) -> ValFilter
{
    f
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action33<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Eq
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action34<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Ne
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action35<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Co
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action36<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Sw
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action37<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Ew
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action38<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Gt
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action39<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Lt
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action40<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Ge
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action41<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompareOp
{
    CompareOp::Le
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action42<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompValue
{
    CompValue::False
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action43<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompValue
{
    CompValue::Null
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action44<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> CompValue
{
    CompValue::True
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action45<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> Result<CompValue,__lalrpop_util::ParseError<usize,Token<'input>,FilterActionError>>
{
    Ok(serde_json::from_str::<serde_json::Number>(n).map(CompValue::Number).map_err(FilterActionError::from)?)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action46<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Result<CompValue,__lalrpop_util::ParseError<usize,Token<'input>,FilterActionError>>
{
    Ok(serde_json::from_str::<String>(s).map(CompValue::Str).map_err(FilterActionError::from)?)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action47<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, a, _): (usize, AttrPath, usize),
) -> PatchPath
{
    PatchPath::Attr(a)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action48<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, a, _): (usize, AttrPath, usize),
    (_, _, _): (usize, (), usize),
    (_, vf, _): (usize, ValFilter, usize),
    (_, _, _): (usize, (), usize),
) -> PatchPath
{
    PatchPath::Value(PatchValuePath {
        attr: a,
        filter: vf,
        sub_attr: None,
    })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action49<
    'input,
    'b,
>(
    budget: &'b ParseBudget,
    input: &'input str,
    (_, a, _): (usize, AttrPath, usize),
    (_, _, _): (usize, (), usize),
    (_, vf, _): (usize, ValFilter, usize),
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, &'input str, usize),
) -> PatchPath
{
    PatchPath::Value(PatchValuePath {
        attr: a,
        filter: vf,
        sub_attr: Some(s.to_string()),
    })
}

#[allow(clippy::type_complexity, dead_code)]
pub trait __ToTriple<'input, 'b, >
{
    fn to_triple(self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>>;
}

impl<'input, 'b, > __ToTriple<'input, 'b, > for (usize, Token<'input>, usize)
{
    fn to_triple(self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>> {
        Ok(self)
    }
}
impl<'input, 'b, > __ToTriple<'input, 'b, > for Result<(usize, Token<'input>, usize), FilterActionError>
{
    fn to_triple(self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, FilterActionError>> {
        self.map_err(|error| __lalrpop_util::ParseError::User { error })
    }
}
