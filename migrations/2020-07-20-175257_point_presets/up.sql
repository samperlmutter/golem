CREATE TABLE point_presets (
    preset_id INT(11) NOT NULL AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    point_quantity INT(11) NOT NULL,
    PRIMARY KEY (preset_id)
);

INSERT INTO point_presets (title, point_quantity) VALUES ("Philanthropy", 100);
INSERT INTO point_presets (title, point_quantity) VALUES ("Rush", 80);
INSERT INTO point_presets (title, point_quantity) VALUES ("Fundraising", 70);
INSERT INTO point_presets (title, point_quantity) VALUES ("Brotherhood/Alumni event", 50);
INSERT INTO point_presets (title, point_quantity) VALUES ("New Member Education (for PNMs)", 50);
INSERT INTO point_presets (title, point_quantity) VALUES ("New Member Education (for Brothers)", 45);
INSERT INTO point_presets (title, point_quantity) VALUES ("Jewish/Hillel (sponsored)", 30);
INSERT INTO point_presets (title, point_quantity) VALUES ("Social", 30);
INSERT INTO point_presets (title, point_quantity) VALUES ("Chapter", 11);

