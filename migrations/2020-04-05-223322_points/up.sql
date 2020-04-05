CREATE TABLE points (
    id INT(11) NOT NULL AUTO_INCREMENT,
    amount INT(11) NOT NULL,
    reason VARCHAR(255) NOT NULL,
    brother_id VARCHAR(50) NOT NULL,
    PRIMARY KEY (id)
);
