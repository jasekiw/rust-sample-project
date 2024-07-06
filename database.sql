CREATE DATABASE RustDatabase;

use RustDatabase;

CREATE TABLE random_numbers(
    Id INT AUTO_INCREMENT PRIMARY KEY,
    Number INT
);

CREATE TABLE current_scale (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    ScaleNumber INT,
    CreatedAt DATETIME
);
