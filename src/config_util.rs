use std::fs;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use schemars::schema::RootSchema;
use serde::{Serialize,Deserialize};
use serde_yaml::from_str as yaml_from_str;
use serde_json::{from_str as json_from_str, to_string_pretty};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig{
  pub cli: String,
  pub scml: String,
  pub kanimal: String
}

pub fn read() -> Option<GlobalConfig>{
  let schema = yaml_from_str::<RootSchema>(
    &read_to_string("config.yml").expect("加载配置文件失败"),
  );
  return match schema {
    Ok(json) => {
      let data = to_string_pretty(&json).expect("配置文件损坏");
      let gc: GlobalConfig = json_from_str(&*data).expect("JSON 转换失败");
      return Some(gc);
    }
    Err(err) => {
      println!("{}", err);
      None
    }
  };
}

pub fn write(){
  println!("{:-^30}","创建配置文件");
  let cli: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("输入完整的 kanimal_cli 路径")
    .interact_text()
    .unwrap();
  let scml: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("输入完整的 scml 存放路径")
    .interact_text()
    .unwrap();
  let kanimal: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("输入完整的 kanimal 文件存放路径")
    .interact_text()
    .unwrap();
  let gc = GlobalConfig{
    cli,
    scml,
    kanimal
  };
  if fs::write(Path::new("config.yml"),serde_yaml::to_string(&gc).unwrap()).is_err() {
    println!("错误：未能写入配置文件");
    exit(4);
  } else {
    println!("成功");
  }
}
