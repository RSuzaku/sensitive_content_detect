use axum::Json;

use crate::{
    core::sensitive_check_core,
    model::sensitive_word,
    ResponseResult,
    response::ResVO,
    init::map_init,
    utils::sensitive_process,
};

use tracing::{info, error};

pub async fn sensitive_check(
payload: Json<sensitive_word::SensitiveContent>) -> ResponseResult<Vec<sensitive_word::SensitiveContentResp>> {
    let text = payload.0.text.unwrap_or_default();

    if text.is_empty() {
        error!("text value is empty");
        let err_resp: ResVO<Vec<sensitive_word::SensitiveContentResp>> = ResVO::from_error(
            Some(1),
            "empty input".to_string(),
            None);
        return Ok(Json(err_resp))
    }
    // 获取停用词
    let stop_word_guard = map_init::STOP_WORD.lock().await;
    let stop_word_set: Vec<char> = stop_word_guard.iter().cloned().collect();

    // 获取字典
    let sensitive_word_guard = map_init::SENSITIVE_MAP.lock().await;
    let sensitive_word_map = sensitive_word_guard.clone();

    let check_sensitive_res = sensitive_check_core::get_sensitive_words(
        &text, sensitive_word::MatchType::MinMatchType,
        &sensitive_word_map, stop_word_set).unwrap();
    let check_res: Vec<sensitive_word::SensitiveContentResp> = sensitive_process::process_sensitive_res(check_sensitive_res)
                .unwrap();
    let resp: ResVO<Vec<sensitive_word::SensitiveContentResp>> = ResVO::success_with_data(Some(check_res));
    info!("sensitive check process successfully!");
    Ok(Json(resp))
}