---
title: 'Rust实现雪花算法生成唯一ID方案'
description: 'Rust实现雪花算法生成唯一ID方案，以及10进制和11-64进制互转'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', 'snow flake', 'uuid']
---

## 起因

发现简书一类的网站资源 ID 很短不知道如何实现

如 https://www.jianshu.com/p/4a8939c48cd4

这个 12 位的

然后就考虑到雪花法但生成结果是：6887650413378670482 近 20 位数字，就考虑是否有其它处理方式

主要目的是想把雪花算法生成的值变得更短

如 6887650413378670482 转换成更短小的格式 16 或 12 位数字字符串

## 实现

经过多方查找现定使用雪花算法生成 ID 再转为 36 进制的 13 位小写字母加数字格式

> 进制转换参考 python 实现方式： https://blog.csdn.net/dutsoft/article/details/79076327

### 初级进制转换

```rust
/// 10 进制转为 11 - 62 进制 36 进制前是小写
fn base_n(num: u64, n: i32) -> String {
    let num_rep: HashMap<i32, char> = HashMap::from([
        (10, 'a'), (11, 'b'), (12, 'c'), (13, 'd'), (14, 'e'),
        (15, 'f'), (16, 'g'), (17, 'h'), (18, 'i'), (19, 'j'),
        (20, 'k'), (21, 'l'), (22, 'm'), (23, 'n'), (24, 'o'),
        (25, 'p'), (26, 'q'), (27, 'r'), (28, 's'), (29, 't'),
        (30, 'u'), (31, 'v'), (32, 'w'), (33, 'x'), (34, 'y'),
        (35, 'z'),
        (36, 'A'), (37, 'B'), (38, 'C'), (39, 'D'), (40, 'E'),
        (41, 'F'), (42, 'G'), (43, 'H'), (44, 'I'), (45, 'J'),
        (46, 'K'), (47, 'L'), (48, 'M'), (49, 'N'), (50, 'O'),
        (51, 'P'), (52, 'Q'), (53, 'R'), (54, 'S'), (55, 'T'),
        (56, 'U'), (57, 'V'), (58, 'W'), (59, 'X'), (60, 'Y'),
        (61, 'Z')
    ]);

    let mut new_num_string = String::from("");
    let mut current: u64 = num;

    while current != 0 {
        let remainder = (current % (n as u64)) as i32;
        let mut remainder_string: String;

        if remainder > 9 && remainder < 62 {
            remainder_string = format!("{}", num_rep.get(&remainder).unwrap());
        } else {
            remainder_string = format!("{}", remainder);
        }

        new_num_string = format!("{}{}", remainder_string, new_num_string);
        current = current / (n as u64);
    }

    new_num_string
}

#[cfg(test)]
mod tests {
    use super::base_n;

    #[test]
    fn test_base_35() {
        let num = 6887946670030594043;
        assert_eq!(base_n(num, 35), "21bx54naqlu18");
    }

    /// 测试16进制结果是否和标准库一致
    #[test]
    fn test_base_16() {
        let num = 6887946670030594043;
        let stand_val = format!("{:x}", num);
        assert_eq!(base_n(num, 16), stand_val);
    }
}
```

### 进阶版进制转换

````rust
const ALL_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";

/// 进阶版 10 进制转 11 - 64 进制
///
/// ```
/// let raw_id = 6888076346770202619;
/// assert_eq!(base_10_to_n(raw_id, 36), "1gbyra5idyk8r");
/// ```
fn base_10_to_n(num: u64, radix: u32) -> String {
    if num == 0 {
        return String::from("0");
    }

    let base = base_10_to_n(num / (radix as u64), radix);
    let start = base.strip_prefix("0").unwrap_or(base.as_str());
    let end = match ALL_CHARS.chars().nth((num % (radix as u64)) as usize) {
        Some(data) => String::from(data),
        _ => String::from(""),
    };
    format!("{}{}", start, end)
}

/// 11 - 64 进制解析为 10 进制
///
/// ```
/// let id = "1gbyra5idyk8r";
/// assert_eq!(base_n_to_10(id, 36), 6888076346770202619);
/// ```
fn base_n_to_10(num_str: &str, radix: u32) -> u128 {
    let mut result: u128 = 0;
    for i in 0..num_str.len() {
        result *= radix as u128;
        let target_char = num_str.chars().nth(i).unwrap_or('0');
        let data = ALL_CHARS.chars().position(|i| i == target_char).unwrap_or(0);
        result += data as u128;
    }
    result
}
````

## 雪花算法终极方案

```rust
use snowflake::SnowflakeIdBucket;

/// 生成雪花算法ID 结果转为36进制
fn get_snow_id(radix: u32) -> (String, u64) {
    let mut b = SnowflakeIdBucket::new(1, 1);
    let raw_id = b.get_id() as u64;
    (base_10_to_n(raw_id, radix), raw_id)
}
```

### 测试雪花算法 ID 生成及进制解析

```rust
use snowflake::SnowflakeIdBucket;
use std::thread::sleep;
use std::time::Duration;

fn test_decode_snow() {
    // 结果转为 36 进制
    let radix = 36;
    for _ in 1..20 {
        let (id, raw_id) = get_snow_id(radix);
        // 进制转换
        let decode_data = base_n_to_10(&id, radix);
        // 输出原始ID  36进制ID  解析为10进制结果
        println!("raw_id: {}  id: {}  de: {}", raw_id, id, decode_data);
        // 暂停1毫秒
        sleep(Duration::from_millis(1));
    }
}
```

### 打印如下结果

```
raw_id: 6888082659701039099  id: 1gbytipmpzgmz  de: 6888082659701039099
raw_id: 6888082659709427707  id: 1gbytipmuz9bv  de: 6888082659709427707
raw_id: 6888082659772342267  id: 1gbytipnwfqij  de: 6888082659772342267
raw_id: 6888082659839451131  id: 1gbytipp0e41n  de: 6888082659839451131
raw_id: 6888082659902365691  id: 1gbytipq1ul8b  de: 6888082659902365691
raw_id: 6888082659969474555  id: 1gbytipr5syrf  de: 6888082659969474555
raw_id: 6888082660032389115  id: 1gbytips79fy3  de: 6888082660032389115
raw_id: 6888082660099497979  id: 1gbytiptb7th7  de: 6888082660099497979
raw_id: 6888082660162412539  id: 1gbytipucoanv  de: 6888082660162412539
raw_id: 6888082660225327099  id: 1gbytipve4ruj  de: 6888082660225327099
```

若有更好的实现方式，欢迎评论指导。
