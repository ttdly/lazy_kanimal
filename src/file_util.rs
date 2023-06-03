use std::collections::HashMap;
use std::ffi::OsStr;
use std::process::exit;
use walkdir::WalkDir;
use regex::Regex;
use crate::config_util::GlobalConfig;

#[derive(Debug)]
pub struct Kanimal{
  pub img: String,
  pub anim: String,
  pub build: String
}

fn os_str_op_to_str (op:Option<&OsStr>) -> &str{
  op.unwrap().to_str().unwrap()
}


pub fn read_kanimal_list(config:&GlobalConfig) -> HashMap<String,Kanimal>{
  let dir = &config.kanimal;
  let mut kanimal_map:HashMap<String,Kanimal> = HashMap::new();
  let anim = Regex::new(r#"_anim$"#).unwrap();
  let build = Regex::new(r#"_build$"#).unwrap();
  let zero = Regex::new(r#"_0$"#).unwrap();
  for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
    if entry.path().is_file() {
      match entry.path().extension() {
        Some(extension) => {
          if extension.to_str().unwrap() == "png" {
            let title = String::from(
              zero.replace(os_str_op_to_str(entry.path().file_stem()),"")
            );
            let file_path = String::from(entry.path().to_str().unwrap());
            match kanimal_map.get_mut(&*title) {
              Some(kanimal) => {
                kanimal.img = file_path;
              },
              None => {
                let k = Kanimal{
                  img: file_path,
                  anim: String::from("null"),
                  build: String::from("null")
                };
                kanimal_map.insert(String::from(title),k);
              }
            }
          } else {
            let file_stem = os_str_op_to_str(entry.path().file_stem());
            let file_path = String::from(entry.path().to_str().unwrap());
            if anim.is_match(file_stem) {
              let title = anim.replace(file_stem,"");
              match kanimal_map.get_mut(&*title) {
                Some(kanimal) => {
                  kanimal.anim = file_path;
                },
                None => {
                  let k = Kanimal{
                    img: String::from("null"),
                    anim: file_path,
                    build: String::from("null")
                  };
                  kanimal_map.insert(String::from(title),k);
                }
              }
            } else if build.is_match(file_stem){
              let title = build.replace(file_stem,"");
              match kanimal_map.get_mut(&*title) {
                Some(kanimal) => {
                  kanimal.build = file_path;
                },
                None => {
                  let k = Kanimal{
                    img: String::from("null"),
                    anim: String::from("null"),
                    build: file_path
                  };
                  kanimal_map.insert(String::from(title),k);
                }
              }
            }
          }
        },
        None => {
          println!("错误：无法解析文件类型");
          exit(5);
        },
      }
    }
  }
  kanimal_map
}
