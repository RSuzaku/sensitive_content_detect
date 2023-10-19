use futures::lock::Mutex;
use lazy_static::lazy_static;

use std::fs::File;
use std::io::Read;
use crate::{
    NewResult,
    model::sensitive_word::*,
};

use tracing::info;

lazy_static! {
    pub static ref SENSITIVE_MAP: Mutex<SensitiveWordMap> = Mutex::new(SensitiveWordMap::new());
    pub static ref STOP_WORD: Mutex<Vec<char>> = Mutex::new(Vec::new());
}



// 读取文件函数
async fn read_file(file_path: &str) -> NewResult<String> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}

// 解析敏感词函数
async fn parse_sensitive_words(content: &str) -> NewResult<Vec<(&str, &str)>> {
    let split_content: Vec<&str> = content.split('\n').collect();

    let mut sensitive_words: Vec<(&str, &str)> = Vec::new();

    for item in split_content {
        if item.is_empty() {
            continue;
        }
        let sensitive_vec: Vec<&str> = item.split('\t').collect();
        if sensitive_vec.len() == 2 && !sensitive_vec[0].is_empty() && !sensitive_vec[1].is_empty(){
            let sensitive_key = sensitive_vec[0];
            let sensitive_value = sensitive_vec[1];
            sensitive_words.push((sensitive_key, sensitive_value));
        } else {
            continue;
        }
    }

    Ok(sensitive_words)
}


pub async fn init_sensitive_word_map() -> NewResult<i32> {
    let sensitive_content = read_file("data/dict.txt").await?;
    let sensitive_word_set: Vec<(&str, &str)> = parse_sensitive_words(&sensitive_content).await?;
    let mut sensitive_word_map = SensitiveWordMap::new();

    for (category, key) in sensitive_word_set {
        let category = category.to_string();
        let key = key.to_string();

        let mut now_map = &mut sensitive_word_map;

        for (i, key_char) in key.chars().enumerate() {
            let word_map = now_map.children.get(&key_char);
            match word_map {
                Some(_child_map) => {
                    now_map = now_map.children.get_mut(&key_char).unwrap();
                }
                None => {
                    let new_word_map = SensitiveWordMap::new();
                    now_map.children.insert(key_char, new_word_map);
                    now_map = now_map.children.get_mut(&key_char).unwrap();
                }
            }

            if i == key.chars().count() - 1 {
                now_map.is_end = true;
                now_map.category = category.clone();
            }
        }
    }
    SENSITIVE_MAP.lock().await.children = sensitive_word_map.children;
    info!("init sensitive map successfully!");
    Ok(0)
}

// 初始化特殊符号
pub async fn init_stop_word() -> NewResult<i32> {
    let stop_word_content = read_file("data/stopword.txt").await?;
    let stop_word_content_vec: Vec<&str> = stop_word_content.lines()
        .map(|res| res.trim_end())
        .collect();

    let stop_word_char_vec: Vec<char>= stop_word_content_vec
        .iter()
        .flat_map(|s| s.chars())
        .collect();
    for item in stop_word_char_vec {
        STOP_WORD.lock().await.push(item);
    }
    info!("init stop word successfully!");
    Ok(0)
}