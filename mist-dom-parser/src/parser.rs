// parser & semantic analyzer

use super::ast::*;
use super::parser_combinator;
use crate::{Result, Error};

#[derive(Debug)]
pub enum ParserError {

}

// 開始タグのパース
pub fn parse_tag_start() -> Result<(U, &'a [T]), E_> {

}
