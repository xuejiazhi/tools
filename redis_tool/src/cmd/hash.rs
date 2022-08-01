use std::collections::HashMap;

use prettytable::{cell, row, Cell, Row, Table};

#[derive(Debug, Clone)]
pub struct HashCMD {}

pub trait Cmd {
    fn hget(&self, key: String, field: String, val: String);
    fn hgetall(&self,data: HashMap<String, String>);
}

impl Cmd for HashCMD {
    fn hget(&self, key: String, field: String, val: String) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["hash-key", "field", "value"]);
        table.add_row(Row::new(vec![
            Cell::new(&key),
            Cell::new(&field),
            Cell::new(&val),
        ]));
        // 打印表格到标准输出
        table.printstd();
    }

    fn hgetall(&self,data: HashMap<String, String>){
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["field", "value"]);
        for (key, val) in data {
            table.add_row(Row::new(vec![Cell::new(&key), Cell::new(&val)]));
        }
        // 打印表格到标准输出
        table.printstd();
    }
}
