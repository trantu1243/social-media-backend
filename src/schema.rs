// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        userid -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        #[max_length = 255]
        imageurl -> Nullable<Varchar>,
        likeid -> Nullable<Array<Nullable<Int4>>>,
        commentid -> Nullable<Array<Nullable<Int4>>>,
        shareid -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        #[max_length = 255]
        background -> Nullable<Varchar>,
        postid -> Nullable<Array<Nullable<Int4>>>,
        followerid -> Nullable<Array<Nullable<Int4>>>,
        followingid -> Nullable<Array<Nullable<Int4>>>,
        likeid -> Nullable<Array<Nullable<Int4>>>,
        commentid -> Nullable<Array<Nullable<Int4>>>,
        shareid -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::joinable!(posts -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
