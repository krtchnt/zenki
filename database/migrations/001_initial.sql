DROP TABLE IF EXISTS transactions, developer_game, purchases, game_user, game_interaction, game_tag, reviews, friends, users, games, developers, tags;
DROP TYPE IF EXISTS rating_n, purchase_n, payment_n;

CREATE TYPE rating_n AS ENUM ('general', 'mature', 'sensitive');
CREATE TYPE purchase_n AS ENUM ('game_purchase', 'in_game_purchase', 'subscriptions', 'DLC', 'etc');
CREATE TYPE payment_n AS ENUM ('credit_card', 'debit_card', 'paypal', 'etc');

CREATE TABLE developers(
    did serial PRIMARY KEY,
    dname VARCHAR(50) NOT NULL,
    descr TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tags(
    tname VARCHAR(50) PRIMARY KEY,
    descr TEXT,
    category VARCHAR(30)
);

CREATE TABLE users(
    uid serial PRIMARY KEY,
    uname VARCHAR(50) NOT NULL UNIQUE,
    passwd VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    bio TEXT,
    birth_date DATE
);

CREATE TABLE games(
    gid serial PRIMARY KEY,
    gname VARCHAR(50) NOT NULL,
    descr TEXT,
    rating rating_n NOT NULL DEFAULT 'general',
    release_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE friends(
    uid int NOT NULL,
    fid int NOT NULL,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    pending boolean NOT NULL DEFAULT TRUE,
    PRIMARY KEY(uid, fid),
    FOREIGN KEY (uid) REFERENCES users(uid) ON DELETE CASCADE,
    FOREIGN KEY (fid) REFERENCES users(uid) ON DELETE CASCADE,
    CHECK (uid != fid)
);

CREATE TABLE reviews(
    rid serial PRIMARY KEY,
    uid int NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
    gid int NOT NULL REFERENCES games(gid) ON DELETE CASCADE,
    rated float CHECK (rated >= 0 AND rated <= 5),
    reviewed_text TEXT,
    reviewed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE game_tag(
    tname VARCHAR(50) NOT NULL,
    gid int NOT NULL,
    PRIMARY KEY(tname, gid),
    FOREIGN KEY (tname) REFERENCES tags(tname) ON DELETE CASCADE,
    FOREIGN KEY (gid) REFERENCES games(gid) ON DELETE CASCADE
);

CREATE TABLE game_interaction(
    uid INT NOT NULL,
    gid INT NOT NULL,
    startplay_at TIMESTAMP NOT NULL,
    duration INTERVAL,
    PRIMARY KEY (uid, gid, startplay_at),
    FOREIGN KEY (uid) REFERENCES users(uid) ON DELETE CASCADE,
    FOREIGN KEY (gid) REFERENCES games(gid) ON DELETE CASCADE
);

CREATE TABLE game_user(
    gid int NOT NULL,
    uid int NOT NULL,
    wishlist boolean DEFAULT FALSE,
    PRIMARY KEY (gid, uid),
    FOREIGN KEY (gid) REFERENCES games(gid) ON DELETE CASCADE,
    FOREIGN KEY (uid) REFERENCES users(uid) ON DELETE CASCADE
);

CREATE TABLE purchases(
    pid serial PRIMARY KEY,
    gid INT NOT NULL REFERENCES games(gid) ON DELETE CASCADE,
    purchase_type purchase_n NOT NULL,
    price float NOT NULL CHECK (price >= 0),
    descr TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE developer_game(
    gid int NOT NULL,
    did int NOT NULL,
    role VARCHAR(30),
    PRIMARY KEY (gid, did),
    FOREIGN KEY (gid) REFERENCES games(gid) ON DELETE CASCADE,
    FOREIGN KEY (did) REFERENCES developers(did) ON DELETE CASCADE
);

CREATE TABLE transactions(
    tid serial PRIMARY KEY,
    uid int NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
    receiver_uid int REFERENCES users(uid) ON DELETE SET NULL,
    pid int NOT NULL REFERENCES purchases(pid) ON DELETE CASCADE,
    payment_method payment_n NOT NULL,
    amount float NOT NULL CHECK (amount > 0),
    bought_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(20) DEFAULT 'completed'
);

CREATE INDEX idx_games_name ON games(gname);
CREATE INDEX idx_games_release ON games(release_at);
CREATE INDEX idx_user_game_wishlist ON game_user(uid) WHERE wishlist = TRUE;
CREATE INDEX idx_transactions_user ON transactions(uid);
