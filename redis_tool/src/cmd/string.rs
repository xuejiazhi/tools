use std::collections::HashMap;

use prettytable::{cell, row, Cell, Row, Table};

#[derive(Debug, Clone)]
pub struct StringCMD {}

pub trait Cmd {
    fn strlen(&self, key: String, length: i32);
    fn mget(&self, data: HashMap<String, String>);
    fn randomkey(&self, key: String, typec: String, ttl: String);
    fn keys(&self, data: Vec<String>);
    fn typec(&self, key: String, typec: String);
    fn get(&self, key: String, val: String, ttl: String, err: String);
    fn set(&self, key: String, val: String, err: String);
    fn opt(&self, walk: String, key: String, err: String);
    fn clear(&self);
}

impl Cmd for StringCMD {
    fn strlen(&self, key: String, length: i32) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["key", "length"]);
        table.add_row(Row::new(vec![
            Cell::new(&key),
            Cell::new(length.to_string().as_str()),
        ]));
        // 打印表格到标准输出
        table.printstd();
    }

    fn mget(&self, data: HashMap<String, String>) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["key", "val"]);
        for (key, val) in data {
            table.add_row(Row::new(vec![Cell::new(&key), Cell::new(&val)]));
        }
        // 打印表格到标准输出
        table.printstd();
    }

    fn randomkey(&self, key: String, typec: String, ttl: String) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["randomkey", "type", "ttl"]);
        table.add_row(Row::new(vec![
            Cell::new(&key),
            Cell::new(&typec),
            Cell::new(&ttl),
        ]));

        // 打印表格到标准输出
        table.printstd();
    }

    //遍历keys出来的数据
    fn keys(&self, data: Vec<String>) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["number", "key"]);
        for x in 0..data.len() {
            let number = (x + 1).to_string();
            table.add_row(Row::new(vec![Cell::new(&number), Cell::new(&data[x])]));
        }

        // 打印表格到标准输出
        table.printstd();
    }

    fn typec(&self, key: String, typec: String) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["key", "type"]);
        table.add_row(Row::new(vec![Cell::new(&key), Cell::new(&typec)]));

        // 打印表格到标准输出
        table.printstd();
    }

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
