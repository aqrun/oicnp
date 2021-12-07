-- Your SQL goes here

INSERT INTO users
    (uid, username, nickname, password, status,
     email, admin, intro, salt)
     VALUES (1, 'admin', 'Admin', '123456', 1, 'admin@abc.com',
             true, 'This is admin', 'abc');