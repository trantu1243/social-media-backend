// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        userid -> Nullable<Int4>,
        postid -> Nullable<Int4>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        avatar_user -> Nullable<Text>,
        content -> Nullable<Text>,
        likeid -> Nullable<Array<Nullable<Int4>>>,
        commentid -> Nullable<Array<Nullable<Int4>>>,
        comment_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        userid -> Nullable<Int4>,
        postid -> Nullable<Int4>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        content -> Nullable<Text>,
        #[max_length = 255]
        avatar_image_url -> Nullable<Varchar>,
        notification_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        userid -> Nullable<Int4>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        avatar_user -> Nullable<Text>,
        content -> Nullable<Text>,
        post_date -> Nullable<Timestamptz>,
        interact_date -> Nullable<Timestamptz>,
        image -> Nullable<Array<Nullable<Text>>>,
        likeid -> Nullable<Array<Nullable<Int4>>>,
        commentid -> Nullable<Array<Nullable<Int4>>>,
        shareid -> Nullable<Array<Nullable<Int4>>>,
        secret -> Nullable<Bool>,
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
        about -> Nullable<Text>,
        avatar -> Nullable<Text>,
        background -> Nullable<Text>,
        photo -> Nullable<Array<Nullable<Text>>>,
        postid -> Nullable<Array<Nullable<Int4>>>,
        followerid -> Nullable<Array<Nullable<Int4>>>,
        followingid -> Nullable<Array<Nullable<Int4>>>,
        likeid -> Nullable<Array<Nullable<Int4>>>,
        commentid -> Nullable<Array<Nullable<Int4>>>,
        shareid -> Nullable<Array<Nullable<Int4>>>,
        notifications -> Nullable<Array<Nullable<Int4>>>,
        checknotification -> Nullable<Bool>,
        blocklist -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::joinable!(comments -> posts (postid));
diesel::joinable!(comments -> users (userid));
diesel::joinable!(notifications -> posts (postid));
diesel::joinable!(notifications -> users (userid));
diesel::joinable!(posts -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    notifications,
    posts,
    users,
);
