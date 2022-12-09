-- Users
CREATE TABLE user_blog (
    id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    pseudo VARCHAR(20) NOT NULL,
    about TEXT,
    inscription_date DATETIME NOT NULL,
    last_connection_date DATETIME NOT NULL,
    hashed_password CHAR(60) NOT NULL
);

-- Session
CREATE TABLE token_session (
    id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    
    token VARCHAR(255) NOT NULL,
    session_name VARCHAR(20),

	FOREIGN KEY (user_id) REFERENCES user_blog(id)
);

-- Permissions
CREATE TABLE role (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name VARCHAR(20) NOT NULL,
    description TEXT
);

CREATE TABLE allowed (
    role_id INT NOT NULL,
    user_id INT NOT NULL,
	FOREIGN KEY (role_id) REFERENCES role(id),
	FOREIGN KEY (user_id) REFERENCES user_blog(id),
    CONSTRAINT PK_linked PRIMARY KEY (role_id, user_id)
);

-- Articles
CREATE TABLE article (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    author_id INT NOT NULL,

    title VARCHAR(100) NOT NULL,
    content MEDIUMTEXT NOT NULL,

    creation_date DATETIME,
    picture_url VARCHAR(100),

    FOREIGN KEY (author_id) REFERENCES user_blog(id)
);

-- Tags
CREATE TABLE tag (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name VARCHAR(20),
    description TEXT
);

CREATE TABLE was (
    tag_id INT NOT NULL,
    article_id INT NOT NULL,
    
    FOREIGN KEY (tag_id) REFERENCES tag(id),
    FOREIGN KEY (article_id) REFERENCES article(id),
    CONSTRAINT PK_was PRIMARY KEY (tag_id, article_id)
);

-- Comments
CREATE TABLE comment (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    content TEXT
);

CREATE TABLE think (
    comment_id INT NOT NULL,
    article_id INT NOT NULL,

    FOREIGN KEY (comment_id) REFERENCES comment(id),
    FOREIGN KEY (article_id) REFERENCES article(id),
    CONSTRAINT PK_think PRIMARY KEY (comment_id, article_id)
);

CREATE TABLE about (
    comment_id INT NOT NULL,
    user_id INT NOT NULL,

    FOREIGN KEY (comment_id) REFERENCES comment(id),
    FOREIGN KEY (user_id) REFERENCES user_blog(id),
    CONSTRAINT PK_think PRIMARY KEY (comment_id, user_id)
);