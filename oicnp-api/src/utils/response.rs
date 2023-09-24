pub fn oic_result_success<T>(data: T) -> OicResult<T> {
    OicResult {
        code: Some(String::from("200")),
        is_success: true,
        data,
        ..OicResult::default()
    }
}

pub fn oic_result_error<T>(code: &str, message: &str) -> OicResult<T> {
    OicResult {
        code: Some(String::from(code)),
        message: Some(String::from(message)),
        is_success: false,
        ..OicResult::default()
    }
}