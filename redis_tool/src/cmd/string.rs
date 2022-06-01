#[macro_use]
use prettytable::{Table, row, Row, Cell, cell};

#[derive(Debug, Clone)]
pub struct StringCMD {
}

pub trait Cmd {
    fn get(&self, key: String, val: String, ttl: String, err: String);
    fn set(&self, key: String, val: String, err: String);
    fn opt(&self, walk: String, key: String, err: String);
    fn clear(&self);
}

impl Cmd for StringCMD {
    fn get(&self, key: String, val: String, ttl: String, err: String) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["key", "val", "ttl", "err"]);
        table.add_row(Row::new(vec![
            Cell::new(&key),
            Cell::new(&val),
            Cell::new(&ttl),
            Cell::new(&err),
        ]));

        // 打印表格到标准输出
        table.printstd();
    }

    fn set(&self, key: String, val: String, err: String) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["walk", "key", "val", "result"]);
        match &err as &str {
            "nil" => {
                table.add_row(Row::new(vec![
                    Cell::new("SET"),
                    Cell::new(&key),
                    Cell::new(&val),
                    Cell::new("Success"),
                ]));
            }
            _ => {
                table.add_row(Row::new(vec![
                    Cell::new("SET"),
                    Cell::new(&key),
                    Cell::new(&val),
                    Cell::new(&err),
                ]));
            }
        }
        // 打印表格到标准输出
        table.printstd();
    }

    fn opt(&self, walk: String, key: String, err: String) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["walk", "key", "result"]);
        match &err as &str {
            "nil" => {
                table.add_row(Row::new(vec![
                    Cell::new(&walk),
                    Cell::new(&key),
                    Cell::new("Success"),
                ]));
            }
            _ => {
                table.add_row(Row::new(vec![
                    Cell::new(&walk),
                    Cell::new(&key),
                    Cell::new(&err),
                ]));
            }
        }
        // 打印表格到标准输出
        table.printstd();
    }

    fn clear(&self) {
        let mut table = Table::new();
        table.add_row(row!["walk", "result"]);
        table.add_row(Row::new(vec![Cell::new("Clear"), Cell::new("Success")]));
    }
}
