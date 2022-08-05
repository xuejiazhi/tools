use crate::util::function;
use std::collections::HashMap;

use prettytable::{cell, row, Cell, Row, Table};

#[derive(Debug, Clone)]
pub struct ListCMD {}

pub trait Cmd {
    fn blpop(&self, data: HashMap<String, String>);
    fn lindex(&self, key: String, indexs: usize, value: String);
    fn llen(&self, key: String, length: i32);
    fn lrange(&self, data: Vec<String>);
    fn l_pub_k_v(&self,header:Vec<String>,data:HashMap<String,String>);
}

impl Cmd for ListCMD {
    fn lrange(&self, data: Vec<String>) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["num", "value"]);
        for i in 0..data.len() {
            table.add_row(Row::new(vec![
                Cell::new(&i.to_string()),
                Cell::new(&data[i]),
            ]));
        }
        // 打印表格到标准输出
        table.printstd();
    }

    fn llen(&self, key: String, length: i32) {
        let length = length.to_string();
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["key", "length"]);
        table.add_row(Row::new(vec![Cell::new(&key), Cell::new(&length)]));
        // 打印表格到标准输出
        table.printstd();
    }

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

    fn l_pub_k_v(&self,header:Vec<String>,data:HashMap<String,String>){
        // 创建表格
        let mut table = Table::new();
        
        //add header
        let mut head = Row::new(vec![]);
        for i in 0..header.len(){
            head.add_cell(Cell::new(&header[i].to_string()));
        }
        table.add_row(head);
        
        for (key, val) in data {
            table.add_row(Row::new(vec![Cell::new(&key), Cell::new(&val)]));
        }
        // 打印表格到标准输出
        table.printstd();
    }
}
