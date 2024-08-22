CREATE DATABASE psicotesis;

CREATE TABLE IF NOT EXISTS participantes
  ( id SERIAL PRIMARY KEY NOT NULL, age int NOT NULL, sex int NOT NULL
  , major varchar(100) NOT NULL, alcohol boolean NOT NULL, alcohol_frequency int
  , drugs boolean NOT NULL, drugs_frequency int, disorder varchar(255)
  , injury boolean NOT NULL, injury_treated boolean, injury_location varchar(255)
  , abuse int NOT NULL, abuse_other varchar(255), shortage int NOT NULL
  , loss int not null, ip_addr varchar(50));

CREATE TABLE IF NOT EXISTS barrat
  ( id int PRIMARY KEY NOT NULL, cognitive int NOT NULL, motor int NOT NULL
  , unplanned int NOT NULL, raw_answers smallint ARRAY[30] NOT NULL
  , FOREIGN KEY(id) REFERENCES participantes(id));

CREATE TABLE IF NOT EXISTS cardsorting
  ( id int PRIMARY KEY NOT NULL, sorting_score int NOT NULL, errors int NOT NULL
  , perseverations int NOT NULL, deferred int NOT NULL, merrors int NOT NULL
  , sorting_ttf int NOT NULL, tae int NOT NULL, game_time int NOT NULL
  , FOREIGN KEY (id) REFERENCES participantes(id));

CREATE TABLE IF NOT EXISTS cardgame
  ( id int PRIMARY KEY NOT NULL, game_score int NOT NULL, answers smallint ARRAY[5] NOT NULL
  , game_ttf int NOT NULL, game_time int NOT NULL, questions smallint ARRAY[3] NOT NULL
  , FOREIGN KEY (id) REFERENCES participantes(id));
