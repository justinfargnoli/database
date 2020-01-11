use sqlparser::ast::*;

fn main() {
    let sql_statments = "INSERT INTO People \
                         VALUES ('Justin');";

    let statments = parse(sql_statments.to_string()).unwrap();

    lowering::perform(statments);
}

pub fn parse(sql_statments: String) -> Result<Vec<Statement>, sqlparser::parser::ParserError> {
    let dialect = sqlparser::dialect::GenericDialect {}; // or AnsiDialect

    sqlparser::parser::Parser::parse_sql(&dialect, sql_statments)
}

mod lowering {
    use sqlparser::ast::Statement;

    pub fn perform(statments: Vec<Statement>) {
        // statments.iter()
    }
}

mod internals {
    use std::collections::{BTreeSet, HashMap};

    struct Database {
        pub database: HashMap<String, Table>,
    }

    struct Table {
        pub table: BTreeSet<Vec<String>>,
    }
}
