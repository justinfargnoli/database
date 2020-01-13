use internals::Database;
use sqlparser::ast::*;

fn main() {
    let sql_statments = "INSERT INTO People \
                         VALUES ('Justin');";

    let mut database: Database = Database::new();

    let statments = parse(sql_statments.to_string()).unwrap();

    lowering::lower(&mut database, statments).unwrap();
}

pub fn parse(sql_statments: String) -> Result<Vec<Statement>, sqlparser::parser::ParserError> {
    let dialect = sqlparser::dialect::GenericDialect {}; // or AnsiDialect

    sqlparser::parser::Parser::parse_sql(&dialect, sql_statments)
}

mod lowering {
    use crate::internals::Database;
    use sqlparser::ast::Statement;

    #[derive(Debug)]
    pub enum LoweringError {
        NotImplmented,
    }

    pub fn lower(database: &mut Database, statements: Vec<Statement>) -> Result<(), LoweringError> {
        for statement in statements {
            match statement {
                Statement::CreateTable {
                    name,
                    columns: _columns,
                    constraints: _constraints,
                    with_options: _with_options,
                    external: _external,
                    file_format: _file_format,
                    location: _location,
                } => create_table(database, name.to_string()),
                _ => return Err(LoweringError::NotImplmented),
            }
        }
        Ok(())
    }

    fn create_table(database: &mut Database, name: String) {
        database.create_table(name)
    }

    #[cfg(test)]
    mod tests {
        use crate::internals::*;
        use crate::lowering::*;
        use crate::*;

        #[test]
        fn create_table() {
            let mut database: Database = Database::new();
            let sql = "CREATE TABLE Person ( \
                       PersonID int, \
                       LastName varchar(255), \
                       FirstName varchar(255), \
                       Address varchar(255), \
                       City varchar(255) \
                       );";
            let statment = parse(sql.to_string()).unwrap();
            lower(&mut database, statment).unwrap();
            assert!(database.table(&"Person".to_string()).is_some())
        }
    }
}

mod internals {
    use std::collections::{BTreeSet, HashMap};

    pub struct Database {
        database: HashMap<String, Table>,
    }

    impl Database {
        pub fn new() -> Database {
            Database {
                database: HashMap::new(),
            }
        }

        pub fn create_table(&mut self, name: String) {
            self.database.insert(name, Table::new());
        }

        pub fn table(&self, name: &str) -> Option<&Table> {
            self.database.get(name)
        }

        pub fn table_mut(&mut self, name: &str) -> Option<&mut Table> {
            self.database.get_mut(name)
        }
    }

    pub struct Table {
        table: BTreeSet<Vec<String>>,
    }

    impl Table {
        pub fn new() -> Table {
            Table {
                table: BTreeSet::new(),
            }
        }
    }
}
