use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone)]
pub struct SensitiveWordMap {
    pub is_end: bool,
    pub category: String,
    pub children: HashMap<char, SensitiveWordMap>,
}

impl SensitiveWordMap {
    pub fn new() -> Self {
        SensitiveWordMap {
            is_end: false,
            category: String::new(),
            children: HashMap::new(),
        }
    }
}

// 定义敏感词匹配规则
#[derive(Debug, PartialEq, Clone)]
pub enum MatchType {
    MinMatchType,
    MaxMatchType,
}


// 请求的content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensitiveContent {
    pub text: Option<String>
}

// 检查结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensitiveContentResp {
    pub category: String,
    pub content: String,
}
