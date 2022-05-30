#[macro_use] use prettytable::{Table, row, Row, Cell, cell};

#[derive(Debug, Clone)]
pub struct StringCMD {
    pub(crate) key: String,
    pub(crate) val: String,
    pub(crate) ttl: String,
    pub(crate) err: String,
}

impl StringCMD {
    pub fn get(&self) {
        // 创建表格
        let mut table = Table::new();
        table.add_row(row!["key", "val", "ttl","err"]);
        table.add_row(Row::new(vec![
            Cell::new(&self.key),
            Cell::new(&self.val),
            Cell::new(&self.ttl),
            Cell::new(&self.err),
        ]));

        // 打印表格到标准输出
        table.printstd();
    }
}
