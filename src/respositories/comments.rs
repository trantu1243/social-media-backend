use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::NewNotification;
use crate::models::Notification;
use crate::schema::notifications;
use crate::schema::posts;
use crate::schema::users;
use crate::{authorization::BearerToken, models::{Comment, CommentInput, NewComment}, schema::comments};


pub struct CommentRespository;

impl CommentRespository {
    pub fn create_comment(c: &mut PgConnection, _auth: BearerToken, comment_input: CommentInput)->QueryResult<Comment>{
        let user = BearerToken::get_user(&_auth, c)?;
        let name_user = user.name.clone();
        let avatar_user = user.avatar.clone();
        let new_comment = NewComment{
            userid: user.id,
            postid: comment_input.postid,
            name: name_user,
            avatar_user: avatar_user.unwrap_or("https://trantu-secret.s3.ap-southeast-2.amazonaws.com/0ce956b2-9787-4756-a580-299568810730.png".to_string()),
            content: comment_input.content
        };
        let comment = diesel::insert_into(comments::table)
            .values(new_comment)
            .returning(Comment::as_returning())
            .get_result::<Comment>(c)?;
    
        let commentids: Option<Vec<Option<i32>>> = posts::table.find(comment_input.postid).select(posts::commentid).get_result(c)?;
     
        let mut comments: Vec<Option<i32>> = commentids.unwrap_or(Vec::new());
        comments.push(Some(comment.id));
        diesel::update(posts::table.find(comment_input.postid)).set(posts::commentid.eq(comments))
        .execute(c)?;

        let new_notification = NewNotification{
            userid: user.id,
            postid: comment_input.postid,
            name: user.name,
            avatar_image_url: user.avatar.unwrap_or("https://trantu-secret.s3.ap-southeast-2.amazonaws.com/0ce956b2-9787-4756-a580-299568810730.png".to_string()),
            content: "commented in your post.".to_string()
        };

        let notification = diesel::insert_into(notifications::table)
        .values(new_notification)
        .returning(Notification::as_returning())
        .get_result::<Notification>(c)?;

        let user_id: Option<i32> = posts::table.find(comment_input.postid).select(posts::userid).get_result(c)?;
        let notification_list: Option<Vec<Option<i32>>> = users::table.find(user_id.unwrap_or(0))
        .select(users::notifications).get_result(c)?;
        let mut notifications: Vec<Option<i32>> = notification_list.unwrap_or(Vec::new());
        notifications.push(Some(notification.id));

        diesel::update(users::table.find(user_id.unwrap_or(0))).set((
            users::notifications.eq(notifications),
            users::checknotification.eq(false)
        ))
        .execute(c)?;
        Ok(comment)
          
    }

    pub fn get_comments(c: &mut PgConnection, _auth: BearerToken, id: i32)->QueryResult<Vec<Comment>>{
        let comments = comments::table.filter(comments::postid.eq(id)).order(comments::comment_date).load::<Comment>(c);
        comments
    }
}
