
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

## 索引

```
-- 1. 按作者查询其所有作品，这是一个非常常见的查询
CREATE INDEX idx_poetry_author_id ON poetry(author_id);

-- 2. 按朝代查询诗词
CREATE INDEX idx_poetry_dynasty ON poetry(dynasty);

-- 3. 按热度排序查询，可以是首页或榜单的常用查询
CREATE INDEX idx_poetry_hot_weight ON poetry(hot_weight DESC);

-- 4. 按创建时间查询最新添加的内容
CREATE INDEX idx_poetry_created_at ON poetry(created_at DESC);

-- 5. 为作者表的姓名创建索引，方便按姓名搜索
CREATE INDEX idx_authors_name ON authors(name);
```