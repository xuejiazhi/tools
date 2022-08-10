use prettytable::{Cell, Row, Table};

#[derive(Debug, Clone)]
pub struct SetCMD {}

pub trait Cmd {
    fn l_pub_k_v(&self, header: Vec<String>, data: Vec<String>);
    fn l_pub_k_mv(&self, header: Vec<String>, data: Vec<Vec<String>>);
}

impl Cmd for SetCMD {
    fn l_pub_k_v(&self, header: Vec<String>, data: Vec<String>) {
        // 创建表格
        let mut table = Table::new();

        //add header
        let mut head = Row::new(vec![]);
        for i in 0..header.len() {
            head.add_cell(Cell::new(&header[i].to_string()));
        }
        table.add_row(head);

        let mut column = Row::new(vec![]);
        for i in 0..data.len() {
            column.add_cell(Cell::new(&data[i].to_string()))
        }

        table.add_row(column);
        // 打印表格到标准输出
        table.printstd();
    }

    fn l_pub_k_mv(&self, header: Vec<String>, data: Vec<Vec<String>>) {
        // 创建表格
        let mut table = Table::new();

        //add header
        let mut head = Row::new(vec![]);
        for i in 0..header.len() {
            head.add_cell(Cell::new(&header[i].to_string()));
        }
        table.add_row(head);

        for i in 0..data.len() {
            let mut column = Row::new(vec![]);
            for j in 0..data[i].len() {
                column.add_cell(Cell::new(&data[i][j].to_string()))
            }
            table.add_row(column);
        }

        // 打印表格到标准输出
        table.printstd();
    }
}
