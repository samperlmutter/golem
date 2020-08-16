CREATE TABLE points (
    id INT(11) NOT NULL AUTO_INCREMENT,
    reason_id INT(11) NOT NULL,
    brother_id VARCHAR(50) NOT NULL,
    `timestamp` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

INSERT INTO points (reason_id, brother_id) VALUES (1, "UN4DNTW5S");
INSERT INTO points (reason_id, brother_id) VALUES (2, "UN4DNTW5S");
INSERT INTO points (reason_id, brother_id) VALUES (2, "UN4DNTW5S");
INSERT INTO points (reason_id, brother_id) VALUES (1, "UN4DNTW5X");
INSERT INTO points (reason_id, brother_id) VALUES (2, "UN4DNTW5U");
INSERT INTO points (reason_id, brother_id) VALUES (1, "UN4DNTW5T");
