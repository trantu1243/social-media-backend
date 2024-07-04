use diesel::{QueryResult, RunQueryDsl};
use rocket::{form::Form, fs::TempFile};
use diesel::prelude::*;

use crate::models::{NewNotification, NewSecretPost, Notification, Post};
use crate::schema::{notifications, users};
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
                    let new_post = diesel::insert_into(posts::table)
                    .values(post)
                    .returning(Post::as_returning())
                    .get_result::<Post>(c);
                    match new_post {
                        Ok(new_post) => {
                            let mut post_id = user.postid.unwrap_or(vec![]);
                            post_id.push(Some(new_post.id));
                            diesel::update(users::table.find(user.id)).set(users::postid.eq(Some(post_id))).execute(c)?;
                            Ok(new_post)
                        },
                        Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
                    }
                },
                Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
            }         
        }).await   
    }

    pub async fn create_secret_post(db: DbConn, _auth: BearerToken, data: Form<PostUploadForm<'_>>,)->QueryResult<Post>{
    
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
                    let post = NewSecretPost {              
                        userid: Some(user.id),
                        name: Some("Anonymous user".to_string()),
                        avatar_user: Some("https://trantu-secret.s3.ap-southeast-2.amazonaws.com/4123763.png".to_string()),
                        content: Some(text),
                        image: Some(urls),
                        secret: Some(true)
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

    pub async fn handle_like(db: DbConn, _auth: BearerToken, id: i32) -> QueryResult<String> {
        db.run(move |c| {
            let user = BearerToken::get_user(&_auth, c)?;

            let likeids: Option<Vec<Option<i32>>> = posts::table.find(id).select(posts::likeid).get_result(c)?;

            let mut new_userids: Vec<Option<i32>> = likeids.unwrap_or(Vec::new());
            if let Some(index) = new_userids.iter().position(|&x| x == Some(user.id)) {
                new_userids.remove(index);
            } else {
                new_userids.push(Some(user.id));
            };
            diesel::update(posts::table.find(id)).set(posts::likeid.eq(new_userids))
            .execute(c)?;

            let user_likeids: Option<Vec<Option<i32>>> = users::table.find(user.id).select(users::likeid).get_result(c)?;    
       
            let mut postids: Vec<Option<i32>> = user_likeids.unwrap_or(Vec::new());
            if let Some(index) = postids.iter().position(|&x| x == Some(id)) {
                postids.remove(index);
            } else {
                postids.push(Some(id));

                let new_notification = NewNotification{
                    userid: user.id,
                    postid: id,
                    name: user.name,
                    avatar_image_url: user.avatar.unwrap_or("https://trantu-secret.s3.ap-southeast-2.amazonaws.com/0ce956b2-9787-4756-a580-299568810730.png".to_string()),
                    content: "liked in your post.".to_string()
                };
        
                let notification = diesel::insert_into(notifications::table)
                .values(new_notification)
                .returning(Notification::as_returning())
                .get_result::<Notification>(c)?;
        
                let user_id: Option<i32> = posts::table.find(id).select(posts::userid).get_result(c)?;
                let notification_list: Option<Vec<Option<i32>>> = users::table.find(user_id.unwrap_or(0))
                .select(users::notifications).get_result(c)?;
                let mut notifications: Vec<Option<i32>> = notification_list.unwrap_or(Vec::new());
                notifications.push(Some(notification.id));
        
                diesel::update(users::table.find(user_id.unwrap_or(0))).set((
                    users::notifications.eq(notifications),
                    users::checknotification.eq(false)
                ))
                .execute(c)?;

            };
            
            let result = diesel::update(users::table.find(user.id)).set(users::likeid.eq(postids))
            .execute(c)?;
            
            Ok(result.to_string())
         
        }).await
    }

    pub fn get_posts(c: &mut PgConnection, _auth: BearerToken)->QueryResult<Vec<i32>>{
        let user = BearerToken::get_user(&_auth, c);
        match user {
            Ok(user) =>{
                let following_list = user.followingid.unwrap_or(Vec::new());
                posts::table.select(posts::id)
                    .filter(posts::userid.eq_any(following_list).and(posts::secret.eq(false)))
                    .order_by(posts::interact_date.desc())
                    .load::<i32>(c)
            },
            Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
        }
    }

    pub fn get_secret_posts(c: &mut PgConnection, _auth: BearerToken)->QueryResult<Vec<i32>>{
        let user = BearerToken::get_user(&_auth, c);
        match user {
            Ok(user) =>{
                posts::table.select(posts::id)
                    .filter(posts::userid.ne(user.id).and(posts::secret.eq(true)))
                    .order_by(posts::interact_date.desc())
                    .load::<i32>(c)
            },
            Err(_) => Err(diesel::result::Error::BrokenTransactionManager)
        }
    }

    pub fn edit_posts(c: &mut PgConnection, _auth: BearerToken){
        
    }
}