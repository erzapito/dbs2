table! {
    music (id) {
        id -> Integer,
        artist -> Varchar,
        disc -> Varchar,
    }
}

table! {
    serie (id) {
        id -> Integer,
        capitulos -> Varchar,
        categoria -> Varchar,
        fansub -> Varchar,
        idioma -> Varchar,
        name -> Varchar,
    }
}

table! {
    wanted (id) {
        id -> Integer,
        artist -> Varchar,
        disc -> Varchar,
        done -> Integer,
        weeks -> Integer,
    }
}
