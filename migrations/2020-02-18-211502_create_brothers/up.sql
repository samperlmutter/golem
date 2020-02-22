CREATE TABLE brothers (
    slack_id VARCHAR(50) NOT NULL,
    can_strike TINYINT(1) NOT NULL,
    can_reset TINYINT(1) NOT NULL,
    `name` VARCHAR(200) NOT NULL,
    points INT(11) NOT NULL,
    PRIMARY KEY (slack_id)
);

INSERT INTO brothers (slack_id, can_strike, can_reset, `name`, points) VALUES ("UN4DNTW5S", 1, 1, "Sam", 0);
