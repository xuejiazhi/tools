use std::{collections::HashMap};
use crate::util::function;

use prettytable::{cell, row, Cell, Row, Table};

#[derive(Debug, Clone)]
pub struct ListCMD {}

pub trait Cmd {
    fn blpop(&self, data: HashMap<String, String>);
    fn lindex(&self, key: String, indexs: usize, value: String);
}

impl Cmd for ListCMD {
    fn lindex(&self, key: String, index: usize, value: String) {
        let index = function::usize_2_string(index);
        // let b = a.to_string();
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["key", "index", "value"]);
        table.add_row(Row::new(vec![
            Cell::new(&key),
            Cell::new(&index),
            Cell::new(&value),
        ]));
        // 打印表格到标准输出
        table.printstd();
    }

    fn blpop(&self, data: HashMap<String, String>) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["list_key", "value"]);
        for (key, val) in data {
            table.add_row(Row::new(vec![Cell::new(&key), Cell::new(&val)]));
        }
        // 打印表格到标准输出
        table.printstd();
    }
}
