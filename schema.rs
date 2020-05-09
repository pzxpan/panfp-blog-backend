table! {
    article (article_id) {
        article_id -> Int4,
        user_id -> Int4,
        title -> Text,
        path -> Text,
        view_count -> Int4,
        comment_count -> Int4,
        like_count -> Int4,
        date -> Timestamptz,
        intro -> Nullable<Text>,
        content_html -> Nullable<Text>,
    }
}

table! {
    article_category (id) {
        article_id -> Int4,
        category_id -> Int4,
        id -> Int4,
    }
}

table! {
    article_label (id) {
        article_id -> Int4,
        label_id -> Int4,
        id -> Int4,
    }
}

table! {
    category (category_id) {
        category_id -> Int4,
        name -> Nullable<Text>,
        category_alias -> Nullable<Text>,
        description -> Nullable<Text>,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    comment (comment_id) {
        comment_id -> Int4,
        user_id -> Int4,
        article_id -> Int4,
        content -> Nullable<Text>,
        comment_count -> Nullable<Int4>,
        like_count -> Nullable<Int4>,
        date -> Timestamptz,
    }
}

table! {
    label (label_id) {
        label_id -> Int4,
        name -> Nullable<Text>,
        label_alias -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

table! {
    user (user_id) {
        user_id -> Int4,
        password -> Nullable<Text>,
        email -> Nullable<Text>,
        register_time -> Timestamptz,
        nick_name -> Nullable<Text>,
        profession -> Nullable<Text>,
        level -> Nullable<Int4>,
        avatar -> Nullable<Text>,
        expire -> Nullable<Timestamptz>,
        login_session -> Nullable<Text>,
    }
}

joinable!(article -> user (user_id));
joinable!(article_category -> category (category_id));
joinable!(article_label -> label (article_id));

allow_tables_to_appear_in_same_query!(
    article,
    article_category,
    article_label,
    category,
    comment,
    label,
    user,
);
