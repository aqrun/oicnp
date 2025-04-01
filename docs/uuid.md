# 雪花ID全新升级终极版

ID生成方式全新升级，nanoID + 13位雪花ID，时间相关的唯一性ID。

## 调用方式

增加 uuid 宏 根据参数个数简化调用方式

```rust 
use oic_core::uuid;

let id1 = uuid!(); // hxivg8x1jk512buew9vj
let id2 = uuid!("i"); // i-hm78byx1jk512buhe68m
let id3 = uuid!("note", 22); // note-48cs215j31jk512buhe68s
 
// 默认长度20位
assert_eq!(id1.len(), 20);

// 前缀是 i-
assert_eq!(id2.starts_with("i-"), true); 

// 前缀是 note-
// 22 加上 前缀一共是27位
assert_eq!(id3.as_str().starts_with("note-"), true); 
assert_eq!(id3.as_str().len(), 27);
```

## `uuid!()` 宏定义

`macro_export` 属性提升宏的作用域为 crate 级

```rust
#[macro_export]
macro_rules! uuid {
    // 生成20位无前缀的雪花ID
    () => {
        $crate::utils::generate_uuid("", 20)
    };

    // 生20位指定前缀的雪花ID
    ($prefix:expr) => {
        $crate::utils::generate_uuid($prefix, 20)
    };

    // 生成指定前缀 和 长度的雪花ID
    ($prefix:expr, $total:expr) => {
        $crate::utils::generate_uuid($prefix, $total)
    };
}
```

## 核心方法实现 `generate_uuid`

```rust
/// 
/// 生成13位无混淆字符雪花ID串
/// 
pub fn snow_id() -> String {
    let radix = 36;
    let raw_id = SNOW_ID_GENERATOR.lock().unwrap().real_time_generate() as u128;

    random_36(raw_id, radix)
}

///
/// 根据 prefix total 生成ID
/// 
pub fn generate_uuid(prefix: &str, total: usize) -> String {
    // 36位无混淆字符集
    let chars = RANDOM_36.chars().collect::<Vec<char>>();
    // 生成13位雪花ID
    let snow = snow_id();
    // 需要的随机字符长度
    let random_len = total - snow.as_str().len();
    let mut random_id = String::from("");

    // 生成指定长度 nanoid
    if random_len > 0 {
        random_id = base_nano::nanoid!(random_len, chars.as_slice());
    }

    // 存在前缀就补充分隔符
    let mut valid_prefix = String::from(prefix);

    if !prefix.is_empty() && !prefix.contains("-") {
        valid_prefix = format!("{}-", prefix);
    }

    format!("{}{}{}", valid_prefix.as_str(), random_id.as_str(), snow.as_str())
}
```

## 测试

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uuid_with_prefix() {
        let id = uuid!("i");
        assert_eq!(id.starts_with("i-"), true);
    }

    #[test]
    fn test_uuid_with_length() {
        let id = uuid!();
        assert_eq!(id.len(), 20);
    }
}
```