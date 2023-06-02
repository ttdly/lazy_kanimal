use std::fs;
use std::path::Path;
use std::process::{Command, exit};
use dialoguer::{Select};
use dialoguer::theme::ColorfulTheme;
use crate::config_util::{read, GlobalConfig, write};
use crate::file_util::read_kanimal_list;

mod config_util;


fn main() {
  let config:GlobalConfig;
  // 判断配置文件是否存在
  if !Path::new("config.yml").exists() {
    // 不存在就创建一个
    write();
  }
  match read() {
    Some(gc) => {
      config = gc;
      loop {
        main_app(&config);
      }
    },
    None => {
      println!("错误：读取配置文件失败");
      exit(4);
    }
  }

}

fn main_app(config:&GlobalConfig){
  let selections = vec![
    "kanimal -> scml",
    "scml -> kanimal",
    "重写配置文件",
    "退出"
  ];
  let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("请选择操作：\n")
    .default(0)
    .items(&selections[..])
    .interact_opt()
    .unwrap();
  match selection {
    Some(0) => {
      scml_kanimal(config);
    },
    Some(1) => {
      println!("选择了2");
    },
    Some(2) => {
      write();
    },
    None => {
      println!("使用回车键确认选择");
    },
    _ =>{
      exit(0);
    }
  }
}

fn scml_kanimal(_config: &GlobalConfig){

}
