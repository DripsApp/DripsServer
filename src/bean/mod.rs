use log::LevelFilter;
use rbatis::Rbatis;

use crate::config;

pub mod comments;
pub mod likes;
pub mod ugc;
pub mod ugc_history;


/// make a sqlite-rbatis
pub async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        config::get_sqlite_path(),
    )
    .unwrap();
    print!("初始化数据库...");
    let sql = std::fs::read_to_string(config::get_sqlite_table_path()).unwrap();
    let raw = fast_log::LOGGER.get_level().clone();
    fast_log::LOGGER.set_level(LevelFilter::Info);
    let _ = rb.exec(&sql, vec![]).await;
    fast_log::LOGGER.set_level(raw);
    println!("[完成]");
    // ------------create tables way 2 end------------

    return rb;
}