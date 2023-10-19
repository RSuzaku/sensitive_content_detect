use lazy_static::lazy_static;

use std::collections::HashMap;

use std::process::exit;
// use std::sync::Mutex;
use futures::lock::Mutex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::NewResult;
use tracing::{
    info,
    error,
};

lazy_static! {
    pub static ref CONFIG: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


// 读取文件,path表示文件路径
async fn read_config(path: &str) -> NewResult<i32> {
	// 读取文件，失败直接返回Err
	let file: File = File::open(path)?;
	let buffered: BufReader<File> = BufReader::new(file);
 	// 存放`[key]`
    let mut key: String = "".to_string();
    // 缓存 : 去掉空格
    let mut new_line = "".to_string();

	for line in buffered.lines().map(|x| x.unwrap()) {
		new_line.clear();
        new_line.push_str(line.trim());
 		// 定义注释为`#`, 遇到注释跳过
 		if line.contains('#') {
            continue;
        } else if line.contains('[') && line.contains(']') { // 解析`[key]`为`key::`
        	key.clear();
            new_line.pop();
            new_line.remove(0);
            key.push_str(new_line.as_str());
            key.push_str("::");
        } else if new_line.contains('=') { // 必须包含表达式, 才将值写入
            let kvs: Vec<&str> = new_line.as_str().split('=').collect::<Vec<&str>>();
            if kvs.len() == 2 { // 如果不满足则定义为异常数据，该数据暂不处理
                // 缓存
                let mut new_key: String = key.clone();
                new_key.push_str(kvs[0]);

                CONFIG.lock().await.insert(new_key.trim().to_string(), kvs[1].trim().to_string());
            }
        }
	}
	Ok(0)
}

// 初始化配置
pub async fn init_config() -> NewResult<i32>{
    let config_path = "conf/app.conf";
    if let Err(err) = read_config(config_path).await {
        error!("error {:?}", err);
        exit(1)
     } else {
        info!("init config successfully!");
        Ok(0)
     }
}
