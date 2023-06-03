use std::fs;
use std::path::Path;
use std::process::{Command, exit, Output};
use dialoguer::{Select};
use dialoguer::theme::ColorfulTheme;

use crate::config_util::{read, GlobalConfig, write};
use crate::file_util::{_read_scml_list, read_kanimal_list};

mod config_util;
mod file_util;

fn banner() {
  println!(r#"
 _
| | __ _ _____   _
| |/ _` |_  / | | |
| | (_| |/ /| |_| |
|_|\__,_/___|\__, |
             |___/
 _               _                 _
| | ____ _ _ __ (_)_ __ ___   __ _| |
| |/ / _` | '_ \| | '_ ` _ \ / _` | |
|   < (_| | | | | | | | | | | (_| | |
|_|\_\__,_|_| |_|_|_| |_| |_|\__,_|_|

                  v1.0.0  MIT  @ttdly"#);
  println!()
}

fn main() {
  banner();
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
    "scml -> kanimal",
    "kanimal -> scml",
    "重写配置文件",
    "退出"
  ];
  let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("请选择操作：")
    .default(0)
    .items(&selections[..])
    .interact_opt()
    .unwrap();
  match selection {
    Some(0) => {
      scml_kanimal(config);
    },
    Some(1) => {
      kanimal_scml(config)
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

fn kanimal_scml(config: &GlobalConfig){
  let kanimal_map = read_kanimal_list(config);
  let mut selections = vec![];
  for (key,_value) in &kanimal_map {
    selections.push(key);
  }

  let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("选择需要转换为 scml 的文件：")
    .items(&selections[..])
    .default(0)
    .interact_opt()
    .unwrap();
  let key = selections[selection.unwrap()];
  let kanimal = kanimal_map.get(key).unwrap();
  let scml_path = Path::new(&config.scml).join(key);
  if scml_path.exists() {
    fs::remove_dir_all(&scml_path).expect("移除目标文件夹失败");
  }
  println!("等待执行结果……\n{:-^30}","=START=");
  let result = Command::new(&config.cli).args([
    "scml",
    &kanimal.img,
    &rename_to_bytes(&kanimal.build),
    &rename_to_bytes(&kanimal.anim),
    "-o",
    scml_path.to_str().unwrap()
  ]).output();

  out_result(result.unwrap())
}

fn rename_to_bytes(file:&String) -> String{
  let path = Path::new(file);
  let extension = path.extension().unwrap().to_str().unwrap();
  if extension == "bytes" {
    return file.clone();
  }
  let bytes = file.replace(extension, "bytes");
  if fs::rename(file, &bytes).is_ok() {
    bytes
  } else {
    println!("文件重命名失败");
    exit(5);
  }
}

fn scml_kanimal(config:&GlobalConfig){
  let scml_map = _read_scml_list(config);
  let mut selections = vec![];
  for (key,_value) in &scml_map {
    selections.push(key);
  }

  let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("选择需要转换为 kanimal 的文件：")
    .items(&selections[..])
    .default(0)
    .interact_opt()
    .unwrap();
  let key = selections[selection.unwrap()];
  let scml = scml_map.get(key).unwrap();
  let kanimal_path = Path::new(&config.kanimal).join(key);
  if kanimal_path.exists() {
    fs::remove_dir_all(&kanimal_path).expect("移除目标文件夹失败");
  }
  println!("等待执行结果……\n{:-^30}","=START=");
  let result = Command::new(&config.cli).args([
    "kanim",
    &scml,
    "-o",
    kanimal_path.to_str().unwrap()
  ]).output();

  out_result(result.unwrap())
}

fn out_result(result:Output){
  if !result.stdout.is_empty() {
    println!("{}",String::from_utf8(result.stdout).unwrap());
  } else {
    println!("错误：kanimal_cli 调用失败\n{}",String::from_utf8(result.stderr).unwrap());
    exit(5);
  }
  println!("{:-^30}","=END=");
}