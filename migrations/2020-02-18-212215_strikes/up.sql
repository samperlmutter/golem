CREATE TABLE strikes (
    id INT(11) NOT NULL AUTO_INCREMENT,
    excusability INT(11) NOT NULL,
    offense INT(11) NOT NULL,
    reason VARCHAR(255) NOT NULL,
    brother_id VARCHAR(50) NOT NULL,
    PRIMARY KEY (id)
);
