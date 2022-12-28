CREATE TABLE `ugc`
(
    `id`            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `title`         TEXT DEFAULT NULL,
    `content`       TEXT NOT NULL,
    `uid`           TEXT DEFAULT 0,
    `likes`         INT DEFAULT 0,
    `comments`      INT DEFAULT 0,
    `created`       INT DEFAULT 0,
    `updated`       INT DEFAULT 0,
    `status`        INT DEFAULT 0
);
CREATE TABLE `ugc_histories`
(
    `id`            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `ref_id`        INT NOT NULL,
    `title`         TEXT DEFAULT NULL,
    `content`       TEXT NOT NULL,
    `created`       INT DEFAULT 0
);
CREATE TABLE `comments`
(
    `id`            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `ref_id`        INT NOT NULL,
    `ref_type`      INT NOT NULL DEFAULT 0,
    `uid`           TEXT DEFAULT 0,
    `content`       TEXT NOT NULL,
    `created`       INT DEFAULT 0,
    `status`        INT DEFAULT 0
);
CREATE TABLE `likes`
(
    `id`            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `ref_id`        INT NOT NULL,
    `ref_type`      INT NOT NULL DEFAULT 0,
    `uid`           TEXT DEFAULT 0,
    `created`       INT DEFAULT 0,
    `deleted`       INT DEFAULT 0
);