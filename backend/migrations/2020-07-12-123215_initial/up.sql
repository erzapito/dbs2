CREATE TABLE `music` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `artist` varchar(255),
  `disc` varchar(255),
  `version` int(11)
);

CREATE TABLE `serie` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `capitulos` varchar(255),
  `categoria` varchar(255),
  `fansub` varchar(255),
  `idioma` varchar(255),
  `name` varchar(255),
  `version` int(11)
);

CREATE TABLE `wanted` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `artist` varchar(255),
  `disc` varchar(255),
  `done` int(11),
  `version` int(11),
  `weeks` int(11)
);
