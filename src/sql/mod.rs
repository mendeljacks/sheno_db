mod parser;
// pub mod tokenizer;

use parser::create::CreateQuery;

use sqlparser::ast::Statement;
use sqlparser::dialect::SQLiteDialect;
use sqlparser::parser::{Parser, ParserError};

use crate::error::{Result, SQLRiteError};

#[derive(Debug, PartialEq)]
pub enum SQLCommand {
    Insert(String),
    Delete(String),
    Update(String),
    CreateTable(String),
    Select(String),
    Unknown(String),
}

impl SQLCommand {
    pub fn new(command: String) -> SQLCommand {
        let v = command.split(" ").collect::<Vec<&str>>();
        match v[0] {
            "insert" => SQLCommand::Insert(command),
            "update" => SQLCommand::Update(command),
            "delete" => SQLCommand::Delete(command),
            "create" => SQLCommand::CreateTable(command),
            "select" => SQLCommand::Select(command),
            _ => SQLCommand::Unknown(command),
        }
    }
}

/// Performs initial parsing of SQL Statement using sqlparser-rs
pub fn process_command(query: &str) -> Result<String> {
    let dialect = SQLiteDialect {};
    let message: String;
    let mut ast = Parser::parse_sql(&dialect, &query).map_err(SQLRiteError::from)?;

    if ast.len() > 1 {
        return Err(SQLRiteError::SqlError(ParserError::ParserError(format!(
            "Expected a single query statement, but there are {}",
            ast.len()
        ))));
    }

    let query = ast.pop().unwrap();

    // Initialy only implementing some basic SQL Statements
    match query {
        Statement::CreateTable { .. } => {
            let result = CreateQuery::new(&query);
            match result {
                Ok(payload) => {
                    println!("Table name: {}", payload.table_name);
                    for col in payload.columns {
                        println!("Column Name: {}, Column Type: {}", col.name, col.datatype);
                    }
                }
                Err(err) => return Err(err),
            }
            message = String::from("CREATE TABLE Statement executed.");
            // TODO: Push table to DB
        }

        // Log _query to console
        Statement::Query(_query) => message = String::from("SELECT Statement executed."),
        Statement::Insert { .. } => message = String::from("INSERT Statement executed."),
        Statement::Delete { .. } => message = String::from("DELETE Statement executed."),
        _ => {
            return Err(SQLRiteError::NotImplemented(
                "SQL Statement not supported yet.".to_string(),
            ))
        }
    };

    Ok(message)
}
