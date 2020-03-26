CREATE TABLE brothers (
    slack_id VARCHAR(50) NOT NULL,
    can_strike TINYINT(1) NOT NULL,
    is_admin TINYINT(1) NOT NULL,
    `name` VARCHAR(200) NOT NULL,
    points INT(11) NOT NULL,
    PRIMARY KEY (slack_id)
);

INSERT INTO brothers (slack_id, can_strike, is_admin, `name`, points) VALUES ("UN4DNTW5S", 1, 1, "Sam", 0);
INSERT INTO brothers (slack_id, can_strike, is_admin, `name`, points) VALUES ("UN4DNTW5T", 0, 1, "Ethan", 0);
INSERT INTO brothers (slack_id, can_strike, is_admin, `name`, points) VALUES ("UN4DNTW5U", 0, 0, "Danny", 0);
