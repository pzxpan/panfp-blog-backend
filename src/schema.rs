table! {
    article (article_id) {
        article_id -> Int4,
        user_id -> Int4,
        title -> Text,
        path -> Nullable<Text>,
        view_count -> Nullable<Int4>,
        comment_count -> Nullable<Int4>,
        like_count -> Nullable<Int4>,
        date -> Nullable<Timestamptz>,
        intro -> Text,
        content_html -> Text,
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
    article_like (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        article_id -> Nullable<Int4>,
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
        date -> Timestamptz,
    }
}

table! {
    hot_category (hot_id) {
        category_id -> Nullable<Int4>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        category_alias -> Nullable<Text>,
        parent_id -> Nullable<Int4>,
        hot_id -> Int4,
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

joinable!(article_label -> label (label_id));

allow_tables_to_appear_in_same_query!(
    article,
    article_category,
    article_label,
    article_like,
    category,
    comment,
    hot_category,
    label,
    user,
);
