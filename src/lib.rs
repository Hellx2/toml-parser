use std::{collections::HashMap, str::FromStr};

use chrono::{ DateTime, FixedOffset, Local, NaiveDate, NaiveTime };

pub struct TOMLData {
    pub items: TOMLObject
}

impl TOMLData {
    pub fn add(&mut self, loc: String, val: TOMLProp) -> Result<(), TOMLError> {
        self.insert(loc, val)
    }

    pub fn insert(&mut self, loc: String, val: TOMLProp) -> Result<(), TOMLError> {
        if let Some(x) = self.items.insert(loc.clone(), val) {
            Err(TOMLError::AlreadyExists(loc, x))
        } else {
            Ok(())
        }
    }
}

pub type TOMLObject = HashMap<String, TOMLProp>;

#[derive(Clone)]
pub enum TOMLProp {
    Item(TOMLItem),
    Object(TOMLObject)
}

#[derive(Clone)]
pub enum TOMLItem {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    OffsetDateTime(DateTime<FixedOffset>),
    LocalDateTime(DateTime<Local>),
    LocalDate(NaiveDate),
    LocalTime(NaiveTime),
    Array(Vec<TOMLItem>),
    Table(TOMLObject),
}

pub enum TOMLError {
    InvalidEscapeSequence(String),
    IntegerOutOfRange(i64),
    InvalidDefinitionOrder(String),
    AlreadyExists(String, TOMLProp),

    // TODO: Add location to SyntaxError
    SyntaxError(&'static str)
}

impl FromStr for TOMLData {
    type Err = TOMLError;

    fn from_str(x: &str) -> Result<TOMLData, Self::Err> {
        let mut main: TOMLData = TOMLData {
            items: HashMap::new()
        };

        let mut open_table: &mut TOMLObject = &mut main.items;

        for line in x.lines() {
            let li = line.trim();
            if let Some(table) = li.strip_prefix('[') {
                if let Some(arr) = li.strip_prefix('[') {
                    // TODO: Make the split return a SyntaxError on failure
                    if let Some(table_name) = arr.split_once("]]") {
                        // TODO: Parse items in table
                        open_table.insert(table_name.0.to_owned(), TOMLProp::Item(TOMLItem::String("".to_string())));
                    } else {
                        // TODO: Add location
                        return Err(TOMLError::SyntaxError("Missing closing ']]' on table."));
                    }
                } else if let Some(table_name) = table.split_once(']') {
                    if main.items.contains_key(table_name.0) {}
                }
            }
        }

        Ok(main)
    }
}

impl From<()> for TOMLData {
    fn from(_: ()) -> TOMLData {
        TOMLData {
            items: HashMap::new()
        }
    }
}
