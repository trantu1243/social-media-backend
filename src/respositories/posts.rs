use diesel::{QueryResult, RunQueryDsl};
use rocket::{form::Form, fs::TempFile};
use diesel::prelude::*;

use crate::models::Post;
use crate::{authorization::BearerToken, aws_s3::AwsS3, models::NewPost, schema::posts, DbConn};

#[derive(FromForm)]
pub struct PostUploadForm<'a> {
    file: TempFile<'a>,
    r#type: &'a str,
    text: &'a str
}

pub struct PostResponsitory;

impl PostResponsitory {
    pub async fn create_post(db: DbConn, _auth: BearerToken, data: Form<PostUploadForm<'_>>,)->QueryResult<Post>{
    
                let mut urls: Vec<Option<String>> = Vec::new();
                let text = data.text.to_string();
                if data.r#type.len() > 0 {
                    let res = AwsS3::handle_file_s3(&data.file, data.r#type).await;
                    match res {
                        Ok(url) =>{    
                            urls.push(Some(url));
                        },
                        Err(_) => {}
                    }
                } 
                
                db.run(move |c|{
                    let user = BearerToken::get_user(&_auth, c);
                    match user {
                        Ok(user) => {
                            let post = NewPost {              
                                userid: Some(user.id),
                                name: Some(user.name),
                                avatar_user: user.avatar,
                                content: Some(text),
                                image: Some(urls)
                            };
                            diesel::insert_into(posts::table)
                            .values(post)
                            .returning(Post::as_returning())
                            .get_result::<Post>(c)
                        },
                        Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
                    }         
                }).await   
    }

    pub fn get_post_from_id(c: &mut PgConnection, id: i32) -> QueryResult<Post> {
        posts::table.find(id).get_result::<Post>(c)
    }
}