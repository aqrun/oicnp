

create table users (
    uid SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    password VARCHAR(64) NOT NULL,
    status SMALLINT NOT NULL DEFAULT 1,
    email VARCHAR(128) NOT NULL,
    admin BOOLEAN DEFAULT false,
    intro VARCHAR DEFAULT '',
    last_login_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    salt VARCHAR(64) DEFAULT '',
    must_change_password boolean DEFAULT false,
    password_changed_on TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN users.gender is '性别 1：男，2：女';
COMMENT ON COLUMN users.admin is '是否管理员';

CREATE TABLE posts (
    pid SERIAL PRIMARY KEY,
    vid VARCHAR NOT NULL
    	CONSTRAINT posts_vid_unique_key UNIQUE,
    uid INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    summary TEXT,
    body TEXT,
    body_format VARCHAR(20),
    deleted BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    updated_by INTEGER
);

CREATE TABLE taxonomies (
    tid SERIAL PRIMARY KEY,
    vid VARCHAR NOT NULL
    	CONSTRAINT taxonomies_vid_unique_key UNIQUE,
    parent_tid INTEGER DEFAULT 0,
    bundle VARCHAR(64) NOT NULL,
    name VARCHAR(128) NOT NULL,
    description VARCHAR NOT NULL
);


