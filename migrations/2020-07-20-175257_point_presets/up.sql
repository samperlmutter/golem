CREATE TABLE point_presets (
    preset_id INT(11) NOT NULL AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    point_quantity INT(11) NOT NULL,
    PRIMARY KEY (preset_id)
);

INSERT INTO point_presets (title, point_quantity) VALUES ("attended chapter", 11);
INSERT INTO point_presets (title, point_quantity) VALUES ("attended brotherhood event", 50);
