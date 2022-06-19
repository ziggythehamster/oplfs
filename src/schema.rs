table! {
    discs (id) {
        id -> Nullable<Integer>,
        title_id -> Text,
        path -> Text,
        media -> Nullable<Text>,
        format -> Nullable<Text>,
        platform -> Nullable<Text>,
        size -> Integer,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        developer -> Nullable<Text>,
        genre -> Nullable<Text>,
        video_mode -> Nullable<Text>,
        release -> Nullable<Date>,
        players -> Nullable<Integer>,
        rating_stars -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
