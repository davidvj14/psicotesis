CREATE TABLE IF NOT EXISTS participantes
  ( id int PRIMARY KEY NOT NULL, age int NOT NULL, sex int NOT NULL
  , major varchar(100) NOT NULL, alcohol boolean NOT NULL, alcohol_frequency int
  , drugs boolean NOT NULL, drugs_frequency int, disorder varchar(255)
  , injury boolean NOT NULL, injury_treated boolean, injury_location varchar(255)
  , abuse int NOT NULL, abuse_other varchar(255), shortage int NOT NULL
  , loss int not null);

CREATE TABLE IF NOT EXISTS barrat
  ( id int PRIMARY KEY NOT NULL, cognitive int NOT NULL, motor int NOT NULL
  , unplanned int NOT NULL, raw_answers char(30) NOT NULL
  , FOREIGN KEY(id) REFERENCES participantes(id));

CREATE TABLE IF NOT EXISTS cardsorting
  ( id int PRIMARY KEY NOT NULL, score int NOT NULL, errors int NOT NULL
  , perseverations int NOT NULL, deferred int NOT NULL, merrors int NOT NULL
  , ttf int NOT NULL, tae int NOT NULL, time int NOT NULL
  , FOREIGN KEY (id) REFERENCES participantes(id));

CREATE TABLE IF NOT EXISTS cardgame
  ( id int PRIMARY KEY NOT NULL, score int NOT NULL, answers smallint ARRAY[5] NOT NULL
  , ttf int NOT NULL, time int NOT NULL, questions smallint ARRAY[3] NOT NULL
  , FOREIGN KEY (id) REFERENCES participantes(id));
