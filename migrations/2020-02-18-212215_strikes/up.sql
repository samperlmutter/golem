CREATE TABLE strikes (
    id INT(11) AUTO_INCREMENT,
    excusability INT(11) NOT NULL,
    offense INT(11) NOT NULL,
    reason VARCHAR(255) NOT NULL,
    brother_id VARCHAR(50) NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO strikes (excusability, offense, reason, brother_id) VALUES (1, 1, "slept in", "UN4DNTW5S");
INSERT INTO strikes (excusability, offense, reason, brother_id) VALUES (1, 1, "skipped", "UN4DNTW5S");
INSERT INTO strikes (excusability, offense, reason, brother_id) VALUES (1, 0, "late", "UN4DNTW5S");
INSERT INTO strikes (excusability, offense, reason, brother_id) VALUES (0, 0, "late", "UN4DNTW5T");
