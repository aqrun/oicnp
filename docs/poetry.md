
## 作者表

```sql
CREATE TABLE IF NOT EXISTS authors (
    id INT AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL DEFAULT '',
    name VARCHAR(100) NOT NULL,
    description TEXT,
    birth_at DATETIME,
    death_at DATETIME,
    dynasty VARCHAR(20) NOT NULL DEFAULT '',
    weight INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
);
```

## 诗词表

```sql
CREATE TABLE IF NOT EXISTS poetry (
    id INT AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL DEFAULT '',
    title VARCHAR(255) NOT NULL,
    author_id INT NOT NULL DEFAULT 0,
    dynasty VARCHAR(20) NOT NULL DEFAULT '',
    weight INT NOT NULL DEFAULT 0,
    hot_weight INT NOT NULL DEFAULT 0,
    content TEXT COMMENT '主内容', -- 这里可以存储整首诗的原文
    word_count INT NOT NULL DEFAULT 0,
    tags VARCHAR(255) NOT NULL DEFAULT '', -- 例如: '思乡,爱情,小学,三年级,婉约'
    FOREIGN KEY (author_id) REFERENCES authors(id)
);

CREATE TABLE IF NOT EXISTS poetry_lines (
    id INT AUTO_INCREMENT PRIMARY KEY,
    poetry_id INT NOT NULL,
    line_number INT NOT NULL, -- 行号，从1开始
    content TEXT NOT NULL, -- 诗句内容
    pinyin TEXT, -- 拼音
    description TEXT, -- 释义
    notes TEXT, -- 单句注释
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (poetry_id) REFERENCES poetry(id),
    UNIQUE KEY (poetry_id, line_number)
);
```

## 章节表

```sql
-- 章节表（存储章节元信息）
CREATE TABLE IF NOT EXISTS chapters (
    id INT AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL DEFAULT '',
    pid INT NOT NULL DEFAULT 0,
    poetry_id INT NOT NULL DEFAULT 0,
    title VARCHAR(255) NOT NULL,
    weight INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 章节内容行表（存储每行内容及扩展信息）
CREATE TABLE IF NOT EXISTS chapter_lines (
    id INT AUTO_INCREMENT PRIMARY KEY,
    chapter_id INT NOT NULL,
    line_number INT NOT NULL,
    content TEXT NOT NULL,
    pinyin TEXT,
    description TEXT, -- 释义
    notes TEXT, -- 单句注释
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id),
    UNIQUE KEY (chapter_id, line_number)
);
```

## 标签

```yaml
poetry:
  desc: 诗词分类
  items:
    dynasty:
      desc: 朝代
      items: [唐,宋,元,明,清,先秦,隋,汉,南北朝,魏,晋,近现代,五代,秦,辽,金,三国]
    theme:
      desc: 主题
      items:
        - 场景:
          items: [送别,咏物,边塞,战争,出游,怀古,咏史,行旅,登高,乡村,宴饮,记梦,贬谪,闺怨,悼亡,题画,唱和,游仙]
        - 情感:
          items: [思乡,爱情,亲情,友情,励志,爱国,同情,忧愁,讽刺,赞美,喜悦,乐观,哲理,惜时,悲愤,感怀,闲愁,悠闲,孤独,怅惘,隐逸,怀才不遇,壮志未酬]
        - 风格:
          items: [豪迈,婉约,浪漫,现实,沉郁,蕴藉,自然,明快,质朴,绮丽,幽默]
        - 景色:
          items: [田园,山水,雨雪,风霜,日月,塞外,园林,古迹]
        - 季节:
          items: [春天,夏天,秋天,冬天]
        - 动植物:
          items: [杏花,柳树,桃树,梅花,兰花,竹子,松树,荷花,桂花,菊,花,草,鸟,鱼,虫]
        - 人物:
          items: [劳动人民,帝王将相,历史人物,神话人物,隐士,战士,儿童,游子巾帼,美人,闺妇,歌女,僧侣]
        - 节日:
          items: [春节,元宵,寒食,清明,端午,七夕,中秋,重阳,腊八]
        - 体裁:
          items: [诗,词,曲,文,乐府,民歌,诗经,楚辞,律诗,绝句,神话,寓言,传,序,记,论,疏,书,赋,说,表,铭]
        - 其他:
          items: [必背古诗词,唐诗三百首,宋词三百首]
    grade:
      desc: 年级
      items: [小学,初中,一年级,二年级,三年级,四年级,五年级,六年级,初一,初二,初三]
```