use crate::{
    config::config::Config,
    parser::{
        self,
        ast::{Ast, Span},
        element::FromAst,
        lexer::Lexer,
    },
};

#[test]
fn test_config() {
    let input = r#"  
        (defwidget foo [arg]
            "heyho")
        (defwidget bar [arg arg2]
            "bla")
        (defvar some_var "bla")
        (defpollvar stuff :interval "12s" "date")
        (deftailvar stuff "tail -f stuff")
    "#;

    let lexer = Lexer::new(0, input.to_string());
    let p = parser::parser::ToplevelParser::new();
    let (span, parse_result) = p.parse(0, lexer).unwrap();
    let config = Config::from_ast(Ast::List(span, parse_result));
    insta::with_settings!({sort_maps => true}, {
        insta::assert_ron_snapshot!(config.unwrap());
    });
}
