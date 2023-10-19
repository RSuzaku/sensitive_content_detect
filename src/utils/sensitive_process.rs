use crate::{
    NewResult,
    model::sensitive_word,
};

// 整合处理结果
pub fn process_sensitive_res(sensitive_res: Vec<String>) -> NewResult<Vec<sensitive_word::SensitiveContentResp>> {
    let mut res: Vec<sensitive_word::SensitiveContentResp> = Vec::new();
    for item in sensitive_res.into_iter() {
        let item_vec:Vec<&str> = item.split(':').into_iter().collect();
        if item_vec.len() != 2 {
            continue;
        }
        let category = item_vec[0].to_string();
        let content: String = item_vec[1].to_string();
        res.push(sensitive_word::SensitiveContentResp {
            category,
            content })
    }
    Ok(res)
}