

create table users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    en_name VARCHAR(128) NOT NULL,
    user_type VARCHAR(20) NOT NULL,
    appears_in VARCHAR DEFAULT '',
    home_planet VARCHAR,
    primary_function VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN users.user_type is '用户类型 1：人类，2：机器人';
COMMENT ON COLUMN users.appears_in is '用户出演的电影系统逗号隔开';

CREATE TABLE user_friends (
    id INTEGER NOT NULL,
    friend_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id, friend_id)
);
COMMENT ON COLUMN user_friends.id is '关联 users.id';
COMMENT ON COLUMN user_friends.friend_id is '关联 users.id';
