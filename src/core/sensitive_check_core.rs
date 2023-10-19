use std::collections::HashSet;

use crate::{
    model::sensitive_word::{MatchType, SensitiveWordMap},
    NewResult,
};

use tracing::{debug, info};

// 筛选敏感词
pub fn get_sensitive_words(
    text: &str,
    match_type: MatchType,
    sensitive_word_map: &SensitiveWordMap,
    stop_word_set: Vec<char>,
) -> NewResult<Vec<String>> {
    info!("get sensitive words process");
    let mut sensitive_word_list = Vec::new();
    let mut i = 0;

    while i < text.len() {
        let (length, category) = check_sensitive_words(
            text,
            i,
            match_type.clone(),
            sensitive_word_map,
            stop_word_set.clone(),
        )
        .unwrap();

        if length > 0 {
            let start = text.char_indices().nth(i).unwrap().0;
            let end = if i + length <= text.len() {
                text.char_indices()
                    .nth(i + length)
                    .map(|(idx, _)| idx)
                    .unwrap_or(text.len())
            } else {
                text.len()
            };
            let word = &text[start..end];
            sensitive_word_list.push(format!("{}:{}", category, word));
            i += length - 1;
        }
        i += 1;
    }
    let unique_vec: Vec<String> = sensitive_word_list
        .iter()
        .cloned()
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();
    info!("get sensitive success!");
    Ok(unique_vec)
}

// 检查敏感词
fn check_sensitive_words(
    target_text: &str,
    begin_index: usize,
    match_type: MatchType,
    sensitive_word_map: &SensitiveWordMap,
    stop_word_set: Vec<char>,
) -> NewResult<(usize, String)> {
    info!("checking sensitive word");
    let mut match_flag: usize = 0; // 敏感词长度
    let mut tmp_flag: usize = 0; // 包括特殊字符的敏感词的长度
    let mut flag = false; // 匹配结果
    let mut category = "".to_string();
    let mut now_map = sensitive_word_map;

    for i in begin_index..target_text.len() {
        let word = match target_text.chars().nth(i) {
            Some(ch) => ch,
            None => {
                // 处理超出范围的情况，例如终止循环或执行其他操作
                break;
            }
        };

        if stop_word_set.contains(&word) && now_map.children.len() < 100 {
            tmp_flag += 1;
            continue;
        }

        now_map = match now_map.children.get(&word) {
            Some(map) => {
                match_flag += 1;
                tmp_flag += 1;
                // 如果为敏感词的最后一个，则退出循环
                if map.is_end {
                    flag = true;
                    category = map.category.clone();
                    // 最小规则，直接返回,最大规则还需继续查找
                    if flag && match_type == MatchType::MinMatchType {
                        break;
                    }
                }
                map
            }
            None => {
                break;
            }
        };
    }

    if match_flag < 2 || !flag {
        // 长度必须大于1。必须为一个词
        tmp_flag = 0;
    }
    info!("sensitive checked successfully!");
    debug!("category assigned {:?}, tmp_flag {:?}", category, tmp_flag);
    Ok((tmp_flag, category))
}
