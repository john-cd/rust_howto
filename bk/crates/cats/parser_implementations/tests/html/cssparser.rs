// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use cssparser::*;

// fn main() {
//     let css_string = "color: red; font-size: 16px;";

//     let mut input = ParserInput::new(css_string);
//     let mut parser = Parser::new(&mut input);

//     while !parser.is_exhausted() {
//         if let Ok(declaration) = parse_declaration(&mut parser) {
//             println!("Property: {}, Value: {}", declaration.0,
// declaration.1);         } else {
//             parser.expect_exhausted(); // handle errors more gracefully in
// real code.             break;
//         }
//     }
// }

// fn parse_declaration<'i, 't>(
//     parser: &mut Parser<'i, 't>,
// ) -> Result<(CowRcStr<'i>, CowRcStr<'i>), ParseError<'i, ()>> {
//     let property = parser.expect_ident()?;
//     parser.expect_colon()?;
//     let value =
// parser.parse_until_comma_or_semicolon(ComponentValue::parse)?;     parser.
// expect_comma_or_semicolon()?;

//     let value_string = value
//         .iter()
//         .map(|component| component.to_css_string())
//         .collect::<String>()
//         .into();

//     Ok((property.clone(), value_string))
// }

// // Implement parsing for ComponentValue (simplified example)
// impl<'i> ToCss for ComponentValue<'i> {
//     fn to_css(&self) -> String {
//         match self {
//             ComponentValue::Ident(ident) => ident.as_ref().to_string(),
//             ComponentValue::Token(token) => token.to_css_string(),
//             _ => "unsupported".to_string(), // Add more cases as needed
//         }
//     }
// }

// impl<'i> ToCss for Token<'i> {
//     fn to_css_string(&self) -> String {
//         match self {
//             Token::Ident(ident) => ident.as_ref().to_string(),
//             Token::Number { value, .. } => value.to_string(),
//             Token::Dimension { value, unit, .. } => {
//                 format!("{}{}", value, unit.as_ref())
//             }
//             Token::Semicolon => ";".to_string(),
//             Token::Colon => ":".to_string(),
//             _ => "unsupported token".to_string(),
//         }
//     }
// }

// impl<'i> ComponentValue<'i> {
//     fn parse<'t>(
//         parser: &mut Parser<'i, 't>,
//     ) -> Result<Self, ParseError<'i, ()>> {
//         if let Ok(ident) = parser.expect_ident() {
//             return Ok(ComponentValue::Ident(ident.clone()));
//         }
//         if let Ok(number) = parser.expect_number() {
//             return Ok(ComponentValue::Token(Token::Number {
//                 value: number.value,
//                 representation: number.representation,
//             }));
//         }
//         if let Ok(dimension) = parser.expect_dimension() {
//             return Ok(ComponentValue::Token(Token::Dimension {
//                 value: dimension.value,
//                 unit: dimension.unit.clone(),
//                 representation: dimension.representation,
//             }));
//         }
//         Err(parser.new_unexpected_token_error(parser.next_token()))
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1092)
