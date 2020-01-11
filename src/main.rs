use sqlparser::ast::*;

fn main() {
    let sql_statments = "INSERT INTO People \
                         VALUES ('Justin');";

    parse(sql_statments.to_string()).unwrap();
}

pub fn parse(sql_statments: String) -> Result<Vec<Statement>, sqlparser::parser::ParserError> {
    let dialect = sqlparser::dialect::GenericDialect {}; // or AnsiDialect

    sqlparser::parser::Parser::parse_sql(&dialect, sql_statments)
}
