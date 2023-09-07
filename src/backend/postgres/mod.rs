pub(crate) mod extension;
pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod types;

use super::*;

/// Postgres query builder.
#[derive(Default, Debug)]
pub struct PostgresQueryBuilder;

const QUOTE: Quote = Quote(b'"', b'"');

impl GenericBuilder for PostgresQueryBuilder {}

impl SchemaBuilder for PostgresQueryBuilder {}

impl QuotedBuilder for PostgresQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for PostgresQueryBuilder {
    /// Escape a SQL string literal
    fn escape_string(&self, string: &str) -> String {
        string
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
            // Note: postgres does not actually support null characters in unicode strings at all
            // .replace('\0', "\\0")
            .replace('\x08', "\\b")
            .replace('\x09', "\\t")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    }

    /// Unescape a SQL string literal
    fn unescape_string(&self, string: &str) -> String {
        let mut escape = false;
        let mut output = String::new();
        for c in string.chars() {
            if !escape && c == '\\' {
                escape = true;
            } else if escape {
                write!(
                    output,
                    "{}",
                    match c {
                        '0' => '\0',
                        'b' => '\x08',
                        't' => '\x09',
                        'n' => '\n',
                        'r' => '\r',
                        c => c,
                    }
                )
                .unwrap();
                escape = false;
            } else {
                write!(output, "{c}").unwrap();
            }
        }
        output
    }
}

impl TableRefBuilder for PostgresQueryBuilder {}
