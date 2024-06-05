use diesel::PgConnection;
use diesel::prelude::*;

use crate::schema::posts;
use crate::{authorization::BearerToken, models::{Comment, CommentInput, NewComment}, schema::comments};


pub struct CommentRespository;

impl CommentRespository {
    pub fn create_comment(c: &mut PgConnection, _auth: BearerToken, comment_input: CommentInput)->QueryResult<Comment>{
        let user = BearerToken::get_user(&_auth, c);
        match user {
            Ok(user) => {
                let new_comment = NewComment{
                    userid: user.id,
                    postid: comment_input.postid,
                    name: user.name,
                    avatar_user: user.avatar.unwrap_or("https://trantu-secret.s3.ap-southeast-2.amazonaws.com/0ce956b2-9787-4756-a580-299568810730.png".to_string()),
                    content: comment_input.content
                };
                let comment = diesel::insert_into(comments::table)
                    .values(new_comment)
                    .returning(Comment::as_returning())
                    .get_result::<Comment>(c);
                match comment {
                    Ok(comment) => {
                        let commentids: Result<Option<Vec<Option<i32>>>, diesel::result::Error> = posts::table.find(comment_input.postid).select(posts::commentid).get_result(c);
                        match commentids {
                            Ok(commentids)=>{
                                let mut comments: Vec<Option<i32>> = commentids.unwrap_or(Vec::new());
                                comments.push(Some(comment.id));
                                diesel::update(posts::table.find(comment_input.postid)).set(posts::commentid.eq(comments))
                                .execute(c)?;
                                Ok(comment)
                            },
                            Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
                        }
                    },
                    Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
                }
            },
            Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
        }
    }
}
