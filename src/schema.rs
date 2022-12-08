table! {
    about (comment_id, user_id) {
        comment_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    allowed (role_id, user_id) {
        role_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    article (id) {
        id -> Integer,
        author_id -> Integer,
        title -> Varchar,
        content -> Mediumtext,
        creation_date -> Nullable<Datetime>,
        picture_url -> Nullable<Varchar>,
    }
}

table! {
    comment (id) {
        id -> Integer,
        content -> Nullable<Text>,
    }
}

table! {
    role (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    tag (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

table! {
    think (comment_id, article_id) {
        comment_id -> Integer,
        article_id -> Integer,
    }
}

table! {
    user_blog (id) {
        id -> Integer,
        pseudo -> Varchar,
        about -> Nullable<Text>,
        inscription_date -> Datetime,
        last_connection_date -> Datetime,
    }
}

table! {
    was (tag_id, article_id) {
        tag_id -> Integer,
        article_id -> Integer,
    }
}

joinable!(about -> comment (comment_id));
joinable!(about -> user_blog (user_id));
joinable!(allowed -> role (role_id));
joinable!(allowed -> user_blog (user_id));
joinable!(article -> user_blog (author_id));
joinable!(think -> article (article_id));
joinable!(think -> comment (comment_id));
joinable!(was -> article (article_id));
joinable!(was -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    about,
    allowed,
    article,
    comment,
    role,
    tag,
    think,
    user_blog,
    was,
);
