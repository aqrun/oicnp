use once_cell::sync::Lazy;
use snowflake::SnowflakeIdBucket;
use snowflake::SnowflakeIdGenerator;
use std::sync::Mutex;

const ALL_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";
const RANDOM_CHARS: &'static str =
    "nXjU7PK3LbqszfmDvpScrHwAhiOk8IeWgVGBoRdZY4JT9MxNuQt05Ea12F6lyC-_";
const RANDOM_36: &'static str = "ms9gwn2xvbr4ioqhy1680j3uzc5eadpkt7fl";

pub static SNOW_ID_GENERATOR: Lazy<Mutex<SnowflakeIdGenerator>> =
    Lazy::new(|| Mutex::new(SnowflakeIdGenerator::new(2, 3)));

pub static SNOW_ID_BUCKET: Lazy<Mutex<SnowflakeIdBucket>> = Lazy::new(|| {
    Mutex::new(SnowflakeIdBucket::new(1, 1))
});

/// 10 进制转 11 - 64 进制
///
/// ```
///  use oicnp_core::utils::base_10_to_n;
/// let raw_id = 6888076346770202619;
/// assert_eq!(base_10_to_n(raw_id, 36), "1gbyra5idyk8r");
/// ```
pub fn base_10_to_n(num: u128, radix: u32) -> String {
    if num == 0 {
        return String::from("0");
    }

    let base = base_10_to_n(num / (radix as u128), radix);
    let start = base.strip_prefix("0").unwrap_or(base.as_str());
    let end = match ALL_CHARS.chars().nth((num % (radix as u128)) as usize) {
        Some(data) => String::from(data),
        _ => String::from(""),
    };
    format!("{}{}", start, end)
}

pub fn random_36(num: u128, radix: u32) -> String {
    if num == 0 {
        return String::from("0");
    }

    let base = base_10_to_n(num / (radix as u128), radix);
    let start = base.strip_prefix("0").unwrap_or(base.as_str());
    let end = match RANDOM_36.chars().nth((num % (radix as u128)) as usize) {
        Some(data) => String::from(data),
        _ => String::from(""),
    };
    format!("{}{}", start, end)
}

pub fn random_base_10_to_n(num: u128, radix: u32) -> String {
    if num == 0 {
        return String::from("0");
    }

    let base = base_10_to_n(num / (radix as u128), radix);
    let start = base.strip_prefix("0").unwrap_or(base.as_str());
    let end = match RANDOM_CHARS.chars().nth((num % (radix as u128)) as usize) {
        Some(data) => String::from(data),
        _ => String::from(""),
    };
    format!("{}{}", start, end)
}

/// 11 - 64 进制解析为 10 进制
///
/// ```
/// use oicnp_core::utils::base_n_to_10;
/// let id = "1gbyra5idyk8r";
/// assert_eq!(base_n_to_10(id, 36) as u64, 6888076346770202619u64);
/// ```
pub fn base_n_to_10(num_str: &str, radix: u32) -> u128 {
    let mut result: u128 = 0;
    for i in 0..num_str.len() {
        result *= radix as u128;
        let target_char = num_str.chars().nth(i).unwrap_or('0');
        let data = ALL_CHARS
            .chars()
            .position(|i| i == target_char)
            .unwrap_or(0);
        result += data as u128;
    }
    result
}

/// 生成雪花算法ID 结果转为36进制
///
/// let (id, raw_id) = generate_snow_id(36);
pub fn generate_snow_id(radix: u32) -> (String, u128) {
    let raw_id = SNOW_ID_BUCKET.lock().unwrap().get_id() as u128;
    (base_10_to_n(raw_id, radix), raw_id)
}

pub fn unique_id(radix: u32) -> (String, u128) {
    let raw_id = SNOW_ID_GENERATOR.lock().unwrap().real_time_generate() as u128;
    (base_10_to_n(raw_id, radix), raw_id)
}

pub fn uuid() -> String {
    let radix = 36;
    let raw_id = SNOW_ID_GENERATOR.lock().unwrap().real_time_generate() as u128;

    random_36(raw_id, radix)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base_35() {
        let num = 6887946670030594043;
        assert_eq!(base_10_to_n(num, 35), "21bx54naqlu18");
    }

    /// 测试16进制结果是否和标准库一致
    #[test]
    fn test_base_16() {
        let num = 6887946670030594043;
        let stand_val = format!("{:x}", num);
        assert_eq!(base_10_to_n(num, 16), stand_val);
    }

    #[test]
    fn test_snow_id_generate_and_decode() {
        let radix = 36;
        let (id, raw_id) = generate_snow_id(radix);
        let decode_id = base_n_to_10(&id, radix);
        assert_eq!(raw_id, decode_id as u128);
    }
}
