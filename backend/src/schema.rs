table! {
    music (id) {
        id -> Integer,
        artist -> Nullable<Text>,
        disc -> Nullable<Text>,
        version -> Nullable<Integer>,
    }
}

table! {
    serie (id) {
        id -> Nullable<Integer>,
        capitulos -> Nullable<Text>,
        categoria -> Nullable<Text>,
        fansub -> Nullable<Text>,
        idioma -> Nullable<Text>,
        name -> Nullable<Text>,
        version -> Nullable<Integer>,
    }
}

table! {
    wanted (id) {
        id -> Nullable<Integer>,
        artist -> Nullable<Text>,
        disc -> Nullable<Text>,
        done -> Nullable<Integer>,
        version -> Nullable<Integer>,
        weeks -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    music,
    serie,
    wanted,
);
