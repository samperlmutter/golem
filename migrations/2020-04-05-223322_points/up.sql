CREATE TABLE points (
    id INT(11) NOT NULL AUTO_INCREMENT,
    amount INT(11) NOT NULL,
    reason VARCHAR(255) NOT NULL,
    brother_id VARCHAR(50) NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO points (amount, reason, brother_id) VALUES (20, "doing something sick", "UN4DNTW5S");
INSERT INTO points (amount, reason, brother_id) VALUES (40, "doing something extra sick", "UN4DNTW5S");
INSERT INTO points (amount, reason, brother_id) VALUES (-100, "doing something very not sick", "UN4DNTW5S");
INSERT INTO points (amount, reason, brother_id) VALUES (-100, "doing something very not sick", "UN4DNTW5X");
INSERT INTO points (amount, reason, brother_id) VALUES (-100, "doing something very very not sick", "UN4DNTW5U");
INSERT INTO points (amount, reason, brother_id) VALUES (100, "being himself", "UN4DNTW5T");
