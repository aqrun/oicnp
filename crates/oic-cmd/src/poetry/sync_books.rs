use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use oic_core::{
    entities::poetry::*,
    models::ModelCrudHandler,
    models::poetry::*,
};
use super::{db_conn, DB};
use crate::utils::format_duration;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct BookData {
    pub chapter: String,
    pub paragraphs: Vec<String>,
}

/// 书籍类内容保存
pub async fn sync_books_data(poetry_dir: &str) -> Result<()> {
    let start_time = std::time::Instant::now();
    sync_si_shu_wu_jing(poetry_dir).await?;
    sync_shijing(poetry_dir).await?;
    sync_lunyu(poetry_dir).await?;
    sync_meng_xue_basic_books(poetry_dir).await?;
    sync_dizigui(poetry_dir).await?;
    sync_guwen_guanzhi(poetry_dir).await?;
    sync_sheng_lv_qi_meng(poetry_dir).await?;
    sync_you_xue_qiong_lin(poetry_dir).await?;
    sync_zeng_guang_xian_wen(poetry_dir).await?;

    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("同步书籍数据完成，耗时: {}", format_duration(duration));
    Ok(())
}

/// 四书五经
/// "name": "四书五经-孟子", 
/// "id": 4,
/// "path": "四书五经/mengzi.json",
/// "data_key": "paragraphs"
pub async fn sync_si_shu_wu_jing(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let author = String::from("孟子");
    let tags = String::from("文言文,四书五经");
    let dynasty = String::from("先秦");
    let daxue_file = format!("{}/四书五经/daxue.json", poetry_dir);
    let zhongyong_file = format!("{}/四书五经/zhongyong.json", poetry_dir);
    // 梦子不太一样
    let mengzi_file = format!("{}/四书五经/mengzi.json", poetry_dir);

    let daxue_content = fs::read_to_string(daxue_file)?;
    let zhongyong_content = fs::read_to_string(zhongyong_file)?;
    let mengzi_content = fs::read_to_string(mengzi_file)?;

    let daxue_data: BookData = serde_json::from_str(&daxue_content)?;
    let zhongyong_data: BookData = serde_json::from_str(&zhongyong_content)?;
    let mengzi_data_list: Vec<BookData> = serde_json::from_str(&mengzi_content)?;

    let author_id = AuthorModel::upsert(db, &CreateAuthorReqParams {
        name: Some(String::from(author.as_str())),
        dynasty: Some(String::from(dynasty.as_str())),
        ..Default::default()
    }).await?;

    PoetryModel::create_multi(db, &[
        CreatePoetryReqParams {
            title: Some(String::from("大学")),
            content: Some(daxue_data.paragraphs.join("\n")),
            author_id: Some(author_id),
            dynasty: Some(String::from(dynasty.as_str())),
            tags: Some(String::from(tags.as_str())),
            weight: Some(1),
            word_count: Some(daxue_data.paragraphs.len() as i32),
            ..Default::default()
        },
        CreatePoetryReqParams {
            title: Some(String::from("中庸")),
            content: Some(zhongyong_data.paragraphs.join("\n")),
            author_id: Some(author_id),
            dynasty: Some(String::from(dynasty.as_str())),
            tags: Some(String::from(tags.as_str())),
            weight: Some(2),
            word_count: Some(zhongyong_data.paragraphs.len() as i32),
            ..Default::default()
        },
    ]).await?;

    println!("四书五经大学、中庸同步完成");

    let word_count = mengzi_data_list.iter().fold(0, |acc, x| {
        acc + x.paragraphs.join("").len()
    });

    let mengzi_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("孟子")),
        content: Some(String::from("book")),
        author_id: Some(author_id),
        dynasty: Some(String::from(dynasty.as_str())),
        tags: Some(String::from(tags.as_str())),
        weight: Some(3),
        word_count: Some(word_count as i32),
        ..Default::default()
    }).await?;

    let mut i = 1;
    let chapters: Vec<CreateChapterReqParams> = mengzi_data_list.iter().map(|x| {
        i += 1;
        let content = x.paragraphs.join("\n");
        CreateChapterReqParams {
            title: Some(String::from(x.chapter.as_str())),
            poetry_id: Some(mengzi_book_id as i32),
            content: Some(String::from(content.as_str())),
            word_count: Some(content.len() as i16),
            weight: Some(i),
            ..Default::default()
        }
    }).collect();

    ChapterModel::create_multi(db, &chapters).await?;

    println!("孟子同步完成");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct ShijingData {
    pub title: String,
    pub chapter: String,
    pub section: String,
    pub content: Vec<String>,
}

pub async fn sync_shijing(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let tags = String::from("文言文,诗经");
    let dynasty = String::from("先秦");
    let shijing_file = format!("{}/诗经/shijing.json", poetry_dir);

    let shijing_content = fs::read_to_string(shijing_file)?;
    let shijing_data: Vec<ShijingData> = serde_json::from_str(&shijing_content)?;

    let word_count = shijing_data.iter().fold(0, |acc, x| {
        acc + x.content.join("").len()
    });

    let shijing_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("诗经")),
        content: Some(String::from("book")),
        word_count: Some(word_count as i32),
        dynasty: Some(String::from(dynasty.as_str())),
        tags: Some(String::from(tags.as_str())),
        description: Some(String::from("《诗经》是中国最早的诗歌总集，收录西周初年至春秋中叶（约公元前11世纪至前6世纪）的诗歌305篇，又称\"诗三百\"。全书分\"风\"\"雅\"\"颂\"三部分：\"风\"为各地民谣，\"雅\"是贵族宴饮乐歌，\"颂\"乃宗庙祭祀乐章。")),
        ..Default::default()
    }).await?;

    let mut book_chapters = PoetryModel::find_all_chapters(db, shijing_book_id as i32).await?;

    let mut i = 1;
    for item in shijing_data {
        let chapter = match book_chapters.iter().find(|x| x.title == item.chapter) {
            Some(x) => x.clone(),
            None => {
                let chapter_id = ChapterModel::create(db, &CreateChapterReqParams {
                    title: Some(String::from(item.chapter.as_str())),
                    poetry_id: Some(shijing_book_id as i32),
                    content: Some(String::from(item.section.as_str())),
                    weight: Some(book_chapters.len() as i16 + 1),
                    ..Default::default()
                }).await?;
                let chapter = ChapterModel::find_by_id(db, chapter_id as i64).await?;
                book_chapters.push(chapter.clone());
                chapter
            }
        };

        ChapterModel::create(db, &CreateChapterReqParams {
            title: Some(String::from(item.title.as_str())),
            poetry_id: Some(shijing_book_id as i32),
            pid: Some(chapter.id as i32),
            content: Some(item.content.join("\n")),
            word_count: Some(item.content.join("").len() as i16),
            weight: Some(i),
            ..Default::default()
        }).await?;

        i += 1;
    }
    println!("诗经同步完成");
    Ok(())
}

pub async fn sync_lunyu(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let tags = String::from("文言文,论语");
    let dynasty = String::from("先秦");
    let lunyu_file = format!("{}/论语/lunyu.json", poetry_dir);

    let lunyu_content = fs::read_to_string(lunyu_file)?;
    let lunyu_data_list: Vec<BookData> = serde_json::from_str(&lunyu_content)?;

    let word_count = lunyu_data_list.iter().fold(0, |acc, x| {
        acc + x.paragraphs.join("").len()
    });

    let lunyu_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("论语")),
        content: Some(String::from("book")),
        word_count: Some(word_count as i32),
        dynasty: Some(String::from(dynasty.as_str())),
        tags: Some(String::from(tags.as_str())),
        description: Some(String::from("《论语》是儒家学派的经典著作之一，由孔子的弟子及其再传弟子编撰而成。它以语录体和对话文体为主，记录了孔子及其弟子言行，集中体现了孔子的政治主张、伦理思想、道德观念及教育原则等。与《大学》《中庸》《孟子》《诗经》《尚书》《礼记》《易经》《春秋》并称“四书五经”。通行本《论语》共二十篇。")),
        ..Default::default()
    }).await?;

    let mut i = 1;
    for item in lunyu_data_list {
        let content = item.paragraphs.join("\n");
        ChapterModel::create(db, &CreateChapterReqParams {
            title: Some(String::from(item.chapter.as_str())),
            poetry_id: Some(lunyu_book_id as i32),
            content: Some(String::from(content.as_str())),
            word_count: Some(content.len() as i16),
            weight: Some(i),
            ..Default::default()
        }).await?;   

        i += 1;
    }
    println!("论语同步完成");
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct BookLoader {
    pub name: String,
    pub path: String,
    pub author: String,
    pub tags: String,
    pub data_key: String,
    pub dynasty: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct MengXueBookData {
    pub title: String,
    pub author: String,
    pub paragraphs: Vec<String>,
}

///
/// 百家姓
/// 千字文
/// 三字经
/// 朱子家训
/// 
pub async fn sync_meng_xue_basic_books(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;

    let loaders: Vec<BookLoader> = vec![
        BookLoader {
            name: String::from("百家姓"),
            path: String::from("蒙学/baijiaxing.json"),
            author: String::from("佚名"),
            tags: String::from("文言文,蒙学"),
            data_key: String::from("paragraphs"),
            dynasty: String::from("北宋"),
        },
        BookLoader {
            name: String::from("千字文"),
            path: String::from("蒙学/qianziwen.json"),
            author: String::from("周興嗣"),
            tags: String::from("文言文,蒙学"),
            data_key: String::from("paragraphs"),
            dynasty: String::from("南北朝"),
        },
        BookLoader {
            name: String::from("三字经"),
            path: String::from("蒙学/sanzijing-new.json"),
            author: String::from("王應麟"),
            tags: String::from("文言文,蒙学"),
            data_key: String::from("paragraphs"),
            dynasty: String::from("宋代"),
        },
        BookLoader {
            name: String::from("朱子家训"),
            path: String::from("蒙学/zhuzijiaxun.json"),
            author: String::from("朱柏廬"),
            tags: String::from("文言文,蒙学"),
            data_key: String::from("paragraphs"),
            dynasty: String::from("清代"),
        },
    ];

    for loader in loaders {
        let file_path = format!("{}/{}", poetry_dir, loader.path);
        let file_content = match fs::read_to_string(file_path.as_str()) {
            Ok(content) => content,
            Err(e) => {
                println!("读取文件失败: {} - {}", e, file_path);
                continue;
            }
        };
        let book_data: MengXueBookData = match serde_json::from_str(&file_content) {
            Ok(data) => data,
            Err(e) => {
                println!("解析文件失败: {} - {}", e, file_path);
                continue;
            }
        };

        let mut author_id = 0;

        if !loader.author.eq("佚名") {
            author_id = match AuthorModel::upsert(db, &CreateAuthorReqParams {
                name: Some(String::from(loader.author.as_str())),
                dynasty: Some(String::from(loader.dynasty.as_str())),
                ..Default::default()
            }).await {
                Ok(id) => id,
                Err(e) => {
                    log::info!("插入作者失败: {} - {}", e, file_path);
                    0
                }
            };
        }

        let content = book_data.paragraphs.join("\n");
        PoetryModel::create(db, &CreatePoetryReqParams {
            title: Some(String::from(book_data.title.as_str())),
            author_id: Some(author_id as i32),
            dynasty: Some(String::from(loader.dynasty.as_str())),
            tags: Some(String::from(loader.tags.as_str())),
            word_count: Some(content.len() as i32),
            content: Some(content),
            ..Default::default()
        }).await?;
    }
    println!("蒙学基础书籍同步完成");
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct DiziguiData {
    pub title: String,
    pub content: Vec<DiziguiChapterData>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct DiziguiChapterData {
    pub chapter: String,
    pub paragraphs: Vec<String>,
}

/// 弟子规
pub async fn sync_dizigui(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let tags = String::from("文言文,蒙学");
    let dynasty = String::from("清代");
    let dizigui_file = format!("{}/蒙学/dizigui.json", poetry_dir);
    let dizigui_content = fs::read_to_string(dizigui_file)?;
    let dizigui_data: DiziguiData = serde_json::from_str(&dizigui_content)?;

    let author_id = AuthorModel::upsert(db, &CreateAuthorReqParams {
        name: Some(String::from("李毓秀")),
        dynasty: Some(String::from(dynasty.as_str())),
        ..Default::default()
    }).await?;

    let word_count = dizigui_data.content.iter().fold(0, |acc, x| {
        acc + x.paragraphs.join("").len()
    });

    let dizigui_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("弟子规")),
        author_id: Some(author_id as i32),
        content: Some(String::from("book")),
        word_count: Some(word_count as i32),
        dynasty: Some(String::from(dynasty.as_str())),
        tags: Some(String::from(tags.as_str())),
        ..Default::default()
    }).await?;

    let mut i = 1;
    for item in dizigui_data.content {
        let content = item.paragraphs.join("\n");
        ChapterModel::create(db, &CreateChapterReqParams {
            title: Some(String::from(item.chapter.as_str())),
            poetry_id: Some(dizigui_book_id as i32),
            word_count: Some(content.len() as i16),
            content: Some(content),
            weight: Some(i),
            ..Default::default()
        }).await?;
        i += 1;
    }
    println!("弟子规同步完成");
    Ok(())
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct GuwenGuanzhiData {
    pub title: String,
    #[serde(rename(deserialize = "abstract"))]
    pub description: Vec<String>,
    pub content: Vec<GuwenGuanzhiContent>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct GuwenGuanzhiContent {
    pub title: String,
    pub content: Vec<GuwenGuanzhiChapterData>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct GuwenGuanzhiChapterData {
    pub chapter: String,
    pub source: String,
    pub author: String,
    pub paragraphs: Vec<String>,
}


/// 古文观止
pub async fn sync_guwen_guanzhi(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let tags = String::from("文言文,蒙学");
    let dynasty = String::from("清代");
    let guwen_guanzhi_file = format!("{}/蒙学/guwenguanzhi.json", poetry_dir);
    let guwen_guanzhi_content = fs::read_to_string(guwen_guanzhi_file)?;
    let guwen_guanzhi_data: GuwenGuanzhiData = serde_json::from_str(&guwen_guanzhi_content)?;

    let word_count = guwen_guanzhi_data.content.iter().fold(0, |acc, x| {
        let chapter_word_count = x.content.iter().fold(0, |m, x| {
            m + x.paragraphs.join("").len()
        });
        acc + chapter_word_count
    });

    let desc = guwen_guanzhi_data.description.join("\n");
    let guwen_guanzhi_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("古文观止")),
        description: Some(desc),
        content: Some(String::from("book")),
        word_count: Some(word_count as i32),
        dynasty: Some(String::from(dynasty.as_str())),
        tags: Some(String::from(tags.as_str())),
        ..Default::default()
    }).await?;

    let mut i = 1;
    for item in guwen_guanzhi_data.content {
        let word_count = item.content.iter().fold(0, |acc, x| {
            acc + x.paragraphs.join("").len()
        });
        let chapter_id = ChapterModel::create(db, &CreateChapterReqParams {
            title: Some(String::from(item.title.as_str())),
            poetry_id: Some(guwen_guanzhi_book_id as i32),
            weight: Some(i),
            word_count: Some(word_count as i16),
            ..Default::default()
        }).await?;

        let mut j = 1;
        for m in item.content {
            let content = m.paragraphs.join("\n");
            ChapterModel::create(db, &CreateChapterReqParams {
                title: Some(String::from(m.chapter.as_str())),
                poetry_id: Some(guwen_guanzhi_book_id as i32),
                pid: Some(chapter_id as i32),
                word_count: Some(content.len() as i16),
                content: Some(content),
                weight: Some(j),
                description: Some(format!("{}|{}", m.author, m.source)),
                ..Default::default()
            }).await?;

            j += 1;
        }

        i += 1;
    }
    println!("古文观止同步完成");
    Ok(())
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct ShengLvQiMengData {
    pub title: String,
    #[serde(rename(deserialize = "abstract"))]
    pub description: String,
    pub content: Vec<ShengLvQiMengContent>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct ShengLvQiMengContent {
    pub title: String,
    pub content: Vec<ShengLvQiMengChapterData>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct ShengLvQiMengChapterData {
    pub chapter: String,
    pub paragraphs: Vec<String>,
}

/// 聲律啓蒙
pub async fn sync_sheng_lv_qi_meng(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let tags = String::from("文言文,蒙学");
    let dynasty = String::from("清");
    let sheng_lv_qi_meng_file = format!("{}/蒙学/shenglvqimeng.json", poetry_dir);
    let sheng_lv_qi_meng_content = fs::read_to_string(sheng_lv_qi_meng_file)?;
    let sheng_lv_qi_meng_data: ShengLvQiMengData = serde_json::from_str(&sheng_lv_qi_meng_content)?;

    let author_id = AuthorModel::upsert(db, &CreateAuthorReqParams {
        name: Some(String::from("车万育")),
        dynasty: Some(String::from(dynasty.as_str())),
        ..Default::default()
    }).await?;

    let word_count = sheng_lv_qi_meng_data.content.iter().fold(0, |acc, x| {
        let chapter_word_count = x.content.iter().fold(0, |m, x| {
            m + x.paragraphs.join("").len()
        });
        acc + chapter_word_count
    });

    let sheng_lv_qi_meng_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("声律启蒙")),
        author_id: Some(author_id as i32),
        description: Some(String::from(sheng_lv_qi_meng_data.description.as_str())),
        content: Some(String::from("book")),
        word_count: Some(word_count as i32),
        dynasty: Some(String::from(dynasty.as_str())),   
        tags: Some(String::from(tags.as_str())),
        ..Default::default()
    }).await?;

    let mut i = 1;
    for item in sheng_lv_qi_meng_data.content {
        let word_count = item.content.iter().fold(0, |acc, x| {
            acc + x.paragraphs.join("").len()
        });
        let chapter_id = ChapterModel::create(db, &CreateChapterReqParams {
            title: Some(String::from(item.title.as_str())),
            poetry_id: Some(sheng_lv_qi_meng_book_id as i32),
            weight: Some(i),
            word_count: Some(word_count as i16),
            ..Default::default()
        }).await?;

        let mut j = 1;
        for m in item.content {
            let content = m.paragraphs.join("\n");
            ChapterModel::create(db, &CreateChapterReqParams {
                title: Some(String::from(m.chapter.as_str())),
                poetry_id: Some(sheng_lv_qi_meng_book_id as i32),
                pid: Some(chapter_id as i32),
                word_count: Some(content.len() as i16),
                content: Some(content),
                weight: Some(j),
                ..Default::default()
            }).await?;

            j += 1;
        }

        i += 1;
    }
    println!("声律启蒙同步完成");
    Ok(())
}

/// 幼学琼林
pub async fn sync_you_xue_qiong_lin(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let tags = String::from("文言文,蒙学");
    let dynasty = String::from("明");
    let file = format!("{}/蒙学/youxueqionglin.json", poetry_dir);
    let content = fs::read_to_string(file)?;
    let you_xue_qiong_lin_data: ShengLvQiMengData = serde_json::from_str(&content)?;

    let author_id = AuthorModel::upsert(db, &CreateAuthorReqParams {
        name: Some(String::from("程登吉")),
        dynasty: Some(String::from(dynasty.as_str())),
        ..Default::default()
    }).await?;

    let word_count = you_xue_qiong_lin_data.content.iter().fold(0, |acc, x| {
        let chapter_word_count = x.content.iter().fold(0, |m, x| {
            m + x.paragraphs.join("").len()
        });
        acc + chapter_word_count
    });

    let sheng_lv_qi_meng_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("幼学琼林")),
        author_id: Some(author_id as i32),
        description: Some(String::from(you_xue_qiong_lin_data.description.as_str())),
        content: Some(String::from("book")),
        word_count: Some(word_count as i32),
        dynasty: Some(String::from(dynasty.as_str())),   
        tags: Some(String::from(tags.as_str())),
        ..Default::default()
    }).await?;

    let mut i = 1;
    for item in you_xue_qiong_lin_data.content {
        let word_count = item.content.iter().fold(0, |acc, x| {
            acc + x.paragraphs.join("").len()
        });
        let chapter_id = ChapterModel::create(db, &CreateChapterReqParams {
            title: Some(String::from(item.title.as_str())),
            poetry_id: Some(sheng_lv_qi_meng_book_id as i32),
            weight: Some(i),
            word_count: Some(word_count as i16),
            ..Default::default()
        }).await?;

        let mut j = 1;
        for m in item.content {
            let content = m.paragraphs.join("\n");
            ChapterModel::create(db, &CreateChapterReqParams {
                title: Some(String::from(m.chapter.as_str())),
                poetry_id: Some(sheng_lv_qi_meng_book_id as i32),
                pid: Some(chapter_id as i32),
                word_count: Some(content.len() as i16),
                content: Some(content),
                weight: Some(j),
                ..Default::default()
            }).await?;

            j += 1;
        }

        i += 1;
    }
    println!("幼学琼林同步完成");
    Ok(())
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct ZengGuangXianWenData {
    pub title: String,
    #[serde(rename(deserialize = "abstract"))]
    pub description: String,
    pub content: Vec<ZengGuangXianChapter>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
struct ZengGuangXianChapter {
    pub chapter: String,
    pub paragraphs: Vec<String>,
}

/// 增广贤文
pub async fn sync_zeng_guang_xian_wen(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let tags = String::from("文言文,蒙学");
    let dynasty = String::from("明");
    let file = format!("{}/蒙学/zengguangxianwen.json", poetry_dir);
    let content = fs::read_to_string(file)?;
    let zeng_guang_xian_wen_data: ZengGuangXianWenData = serde_json::from_str(&content)?;

    let word_count = zeng_guang_xian_wen_data.content.iter().fold(0, |acc, x| {
        acc + x.paragraphs.join("").len()
    });

    let sheng_lv_qi_meng_book_id = PoetryModel::create(db, &CreatePoetryReqParams {
        title: Some(String::from("增广贤文")),
        description: Some(String::from(zeng_guang_xian_wen_data.description.as_str())),
        content: Some(String::from("book")),
        word_count: Some(word_count as i32),
        dynasty: Some(String::from(dynasty.as_str())),   
        tags: Some(String::from(tags.as_str())),
        ..Default::default()
    }).await?;

    let mut i = 1;
    for item in zeng_guang_xian_wen_data.content {
        let content = item.paragraphs.join("\n");
        let _ = ChapterModel::create(db, &CreateChapterReqParams {
            title: Some(String::from(item.chapter.as_str())),
            poetry_id: Some(sheng_lv_qi_meng_book_id as i32),
            weight: Some(i),
            word_count: Some(content.len() as i16),
            content: Some(content),
            ..Default::default()
        }).await?;

        i += 1;
    }
    println!("增广贤文同步完成");
    Ok(())
}