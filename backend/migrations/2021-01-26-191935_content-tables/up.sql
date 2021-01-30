-- Your SQL goes here
CREATE TABLE groups (
    name VARCHAR PRIMARY KEY
);

CREATE TABLE content (
    groupname VARCHAR references groups(name),
    page VARCHAR NOT NULL,
    contentbody VARCHAR,
    PRIMARY KEY(groupname, page)
);

CREATE TABLE privillege (
    user_id SERIAL references users(id),
    groupname VARCHAR references groups(name),
    rights CHAR(4) NOT NULL,
    PRIMARY KEY(user_id, groupname)
);

