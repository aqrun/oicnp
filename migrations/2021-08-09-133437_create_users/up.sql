CREATE TABLE file (
    fid SERIAL PRIMARY KEY,
    uid INTEGER NOT NULL,
    filename VARCHAR NOT NULL,
    uri VARCHAR NOT NULL,
    storage VARCHAR(64) NOT NULL,
    mime VARCHAR NOT NULL DEFAULT '',
    site BIGINT NOT NULL DEFAULT 0,
    status smallint NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN file.storage is '资源存储位置类型如 local,qiniu(七牛),oos(阿里OOS)';
CREATE INDEX file_uri__idx ON file (uri);

CREATE TABLE users (
    uid SERIAL PRIMARY KEY,
    username VARCHAR(128) NOT NULL
        CONSTRAINT user_username_unique_key UNIQUE,
    nickname VARCHAR NOT NULL DEFAULT '',
    password VARCHAR(64) NOT NULL,
    status SMALLINT NOT NULL DEFAULT 1,
    email VARCHAR(128) NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT false,
    intro VARCHAR NOT NULL DEFAULT '',
    last_login_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    salt VARCHAR(64) NOT NULL DEFAULT '',
    must_change_password BOOLEAN NOT NULL DEFAULT false,
    password_changed_on INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN users.admin is '是否管理员';

CREATE TABLE user_picture (
    bundle VARCHAR(20) NOT NULL,
    uid INTEGER NOT NULL,
    fid INTEGER NOT NULL,
    weight INTEGER NOT NULL DEFAULT 0,
    alt VARCHAR(512) NOT NULL DEFAULT '',
    title VARCHAR(1024) NOT NULL DEFAULT '',
    width BIGINT NOT NULL DEFAULT 0,
    height BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT user_picture__pkey
        PRIMARY KEY (uid, fid)
);

COMMENT ON COLUMN user_picture.bundle is '图片类型 avatar';

CREATE TABLE taxonomy (
    tid SERIAL PRIMARY KEY,
    vid VARCHAR NOT NULL,
    pid INTEGER NOT NULL DEFAULT 0,
    bundle VARCHAR(64) NOT NULL,
    name VARCHAR(128) NOT NULL,
    description VARCHAR NOT NULL,
    description_format VARCHAR(20) NOT NULL DEFAULT '',
    weight INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT taxonomy_vid_bundle_unique
        UNIQUE (vid, bundle)
);

COMMENT ON COLUMN taxonomy.bundle is '资源类型如 category, tag';
COMMENT ON COLUMN taxonomy.description_format is '内容类型如 html, markdown, text';

CREATE INDEX taxonomy__vid_name__idx
    ON taxonomy (vid, name);

CREATE TABLE comment (
    cid SERIAL PRIMARY KEY,
    uid BIGINT NOT NULL,
    pid BIGINT NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 1,
    bundle VARCHAR(128) NOT NULL,
    target_id BIGINT NOT NULL,
    subject VARCHAR NOT NULL,
    name VARCHAR(128) NOT NULL DEFAULT '',
    email VARCHAR(128) NOT NULL DEFAULT '',
    homepage VARCHAR(128) NOT NULL DEFAULT '',
    hostname VARCHAR(128) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL DEFAULT 0
);

COMMENT ON COLUMN comment.bundle is '评论对象类型，如 node.article,node.page';
CREATE INDEX comment_bundle_target_id___idx ON comment(bundle, target_id);

CREATE TABLE comment_body (
    cid BIGINT NOT NULL,
    body TEXT,
    body_format VARCHAR(20) NOT NULL DEFAULT '',
    CONSTRAINT comment_body__pkey
        PRIMARY KEY (cid)
);

CREATE TABLE node (
    nid SERIAL PRIMARY KEY,
    vid VARCHAR NOT NULL
    	CONSTRAINT posts_vid_unique_key UNIQUE,
    uid INTEGER NOT NULL,
    bundle VARCHAR(128) NOT NULL,
    title VARCHAR NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT false,
    published_at INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL DEFAULT 0
);

COMMENT ON COLUMN node.bundle is '内容类型如 article, page';

CREATE TABLE node_body (
    nid INTEGER NOT NULL,
    summary TEXT,
    body TEXT,
    body_format VARCHAR(20) NOT NULL DEFAULT '',
    CONSTRAINT node_body__pkey
        PRIMARY KEY (nid)
);

COMMENT ON COLUMN node_body.body_format is '内容类型如 html, markdown, text';

CREATE TABLE node_category_map (
    bundle VARCHAR(20) NOT NULL,
    nid INTEGER NOT NULL,
    tid INTEGER NOT NULL,
    constraint node_category_map___pkey
        PRIMARY KEY (nid, tid)
);

COMMENT ON COLUMN node_category_map.bundle IS '资源类型 如 article, page';

CREATE TABLE node_tags_map (
    bundle VARCHAR(20) NOT NULL,
    nid INTEGER NOT NULL,
    tid INTEGER NOT NULL,
    CONSTRAINT node_tags_map___pkey
        PRIMARY KEY (nid, tid)
);

COMMENT ON COLUMN node_tags_map.bundle IS '资源类型 如 article, page';

CREATE TABLE node_images_map (
    bundle VARCHAR(20) NOT NULL,
    nid INTEGER NOT NULL,
    fid INTEGER NOT NULL,
    weight INTEGER NOT NULL DEFAULT 0,
    alt VARCHAR(512) NOT NULL DEFAULT '',
    title VARCHAR(1024) NOT NULL DEFAULT '',
    width INTEGER NOT NULL DEFAULT 0,
    height INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT node_images_map___pkey
        PRIMARY KEY (nid, fid)
);

CREATE TABLE node_comments_map (
    bundle VARCHAR(20) NOT NULL,
    nid INTEGER NOT NULL,
    cid BIGINT NOT NULL,
    CONSTRAINT node_comments_map___pkey
        PRIMARY KEY (nid, cid)
);

CREATE TABLE config (
    name VARCHAR NOT NULL,
    data VARCHAR NOT NULL,
    CONSTRAINT config___pkey PRIMARY KEY(name)
);




