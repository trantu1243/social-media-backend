use crate::diesel::RunQueryDsl;
use crate::diesel::SelectableHelper;
use crate::models::FriendRequest;
use crate::models::NewFriendRequest;
use crate::models::SearchUser;
use crate::schema::users;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::QueryResult;

use crate::{authorization::BearerToken, schema::friend_requests};

#[derive(serde::Deserialize)]
pub struct FriendRqInput {
    pub id1: i32,
    pub id2: i32,
}

pub struct FriendRequestRespository;

impl FriendRequestRespository {
    pub fn create_request(c: &mut PgConnection, _auth: BearerToken, fr_rq: FriendRqInput)->QueryResult<FriendRequest>{
        if fr_rq.id1 != fr_rq.id2 {
            let id_rq = friend_requests::table.select(friend_requests::id)
            .filter(
                (friend_requests::userid1.eq(Some(fr_rq.id1)).and(friend_requests::userid2.eq(Some(fr_rq.id2))))
                .or(friend_requests::userid1.eq(Some(fr_rq.id2)).and(friend_requests::userid2.eq(Some(fr_rq.id1))))
            )
            .first::<i32>(c);
            match id_rq {
                Ok(_) => Err(diesel::result::Error::BrokenTransactionManager),
                Err(_) => {
                    let following_list: Option<Vec<Option<i32>>> = users::table.find(fr_rq.id1).select(users::followingid).get_result(c)?;
                    let new_value1: Option<i32> = Some(fr_rq.id2); 
                    let updated_following_list = match following_list {
                        Some(mut vec) => {
                            vec.push(new_value1);
                            Some(vec)
                        },
                        None => {
                            Some(vec![new_value1])
                        },
                    };
                    diesel::update(users::table.find(fr_rq.id1)).set(users::followingid.eq(updated_following_list))
                    .execute(c)?;


                    let follower_list: Option<Vec<Option<i32>>> = users::table.find(fr_rq.id2).select(users::followerid).get_result(c)?;
                    let new_value2: Option<i32> = Some(fr_rq.id1);
                    let updated_follower_list = match follower_list {
                        Some(mut vec) => {
                            vec.push(new_value2);
                            Some(vec)
                        },
                        None => {
                            Some(vec![new_value2])
                        },
                    };
                    diesel::update(users::table.find(fr_rq.id2)).set(users::followerid.eq(updated_follower_list))
                    .execute(c)?;

                    let new_rq = NewFriendRequest { 
                        userid1: Some(fr_rq.id1), 
                        userid2: Some(fr_rq.id2) 
                    };
                    diesel::insert_into(friend_requests::table)
                    .values(new_rq)
                    .returning(FriendRequest::as_returning())
                    .get_result::<FriendRequest>(c)
                }
            }
        } 
        else {
            Err(diesel::result::Error::BrokenTransactionManager)
        }
    }

    pub fn check_request(c: &mut PgConnection, _auth: BearerToken, id: i32)->QueryResult<FriendRequest>{
        let user = BearerToken::get_user(&_auth, c);
        match user {
            Ok(user)=>{
                friend_requests::table.filter(
                    (friend_requests::userid1.eq(Some(user.id)).and(friend_requests::userid2.eq(Some(id))))
                    .or(friend_requests::userid1.eq(Some(id)).and(friend_requests::userid2.eq(Some(user.id))))
                )
                .first::<FriendRequest>(c)
            },
            Err(_)=>Err(diesel::result::Error::BrokenTransactionManager)
        }
    }

    pub fn confirm_request(c: &mut PgConnection, _auth: BearerToken, id: i32)->QueryResult<String>{
        let user = _auth.get_user(c)?;
        let id_rq = friend_requests::table.select(friend_requests::id)
        .filter(
            friend_requests::userid1.eq(Some(id)).and(friend_requests::userid2.eq(user.id))
        )
        .first::<i32>(c);
        match id_rq {
            Ok(_) => {
                let following_list: Option<Vec<Option<i32>>> = users::table.find(user.id).select(users::followingid).get_result(c)?;
                let new_value1: Option<i32> = Some(id); 
                let updated_following_list = match following_list {
                    Some(mut vec) => {
                        vec.push(new_value1);
                        Some(vec)
                    },
                    None => {
                        Some(vec![new_value1])
                    },
                };
                diesel::update(users::table.find(user.id)).set(users::followingid.eq(updated_following_list))
                .execute(c)?;

                let follower_list: Option<Vec<Option<i32>>> = users::table.find(id).select(users::followerid).get_result(c)?;
                let new_value2: Option<i32> = Some(user.id);
                let updated_follower_list = match follower_list {
                    Some(mut vec) => {
                        vec.push(new_value2);
                        Some(vec)
                    },
                    None => {
                        Some(vec![new_value2])
                    },
                };
                diesel::update(users::table.find(id)).set(users::followerid.eq(updated_follower_list))
                .execute(c)?;
                
                diesel::update(friend_requests::table.filter( friend_requests::userid1.eq(Some(id)).and(friend_requests::userid2.eq(user.id))))
                .set(friend_requests::confirm.eq(Some(true)))
                .execute(c)?;
                Ok("Success".to_string())
            },
            Err(_) =>Err(diesel::result::Error::BrokenTransactionManager) 
        }
    }

    pub fn delete_request(c: &mut PgConnection, _auth: BearerToken, id: i32)->QueryResult<String>{
        let user = _auth.get_user(c)?;
        diesel::delete(friend_requests::table.filter(
            (friend_requests::userid1.eq(Some(user.id)).and(friend_requests::userid2.eq(Some(id))))
                .or(friend_requests::userid1.eq(Some(id)).and(friend_requests::userid2.eq(Some(user.id))))
        )).execute(c)?;
        Ok("Success".to_string())
    }

    pub fn get_friend_request(c: &mut PgConnection, _auth: BearerToken)->QueryResult<Vec<SearchUser>>{
        let user = _auth.get_user(c)?;
        let requests: Result<Vec<FriendRequest>, _> = friend_requests::table.filter(
            (friend_requests::userid2.eq(user.id)).and(friend_requests::confirm.eq(false))
        ).load::<FriendRequest>(c);
        let mut list: Vec<SearchUser> = Vec::new();
        match requests {
            Ok(requests) => {
                for request in requests {
                    let friend: SearchUser = users::table.select((
                        users::id,
                        users::name,
                        users::about,
                        users::avatar,
                        users::followerid
                    )).find(request.userid1.unwrap_or(0))
                    .first::<SearchUser>(c)?;
                    list.push(friend);
                }
                Ok(list)
            },
            Err(_)=>Err(diesel::result::Error::BrokenTransactionManager) 
        }
    }

    pub fn get_friends_of_user(c: &mut PgConnection, auth: BearerToken)->QueryResult<Vec<SearchUser>>{
        let user = auth.get_user(c)?;
        let requests: Result<Vec<FriendRequest>, _> = friend_requests::table.filter(
            ((friend_requests::userid1.eq(user.id)).or(friend_requests::userid2.eq(user.id))).and(friend_requests::confirm.eq(true))
        ).load::<FriendRequest>(c);
        let mut list: Vec<SearchUser> = Vec::new();
        match requests {
            Ok(requests)=>{
                for request in requests {
                    if request.userid1 == Some(user.id) {
                        let friend: SearchUser = users::table.select((
                            users::id,
                            users::name,
                            users::about,
                            users::avatar,
                            users::followerid
                        )).find(request.userid2.unwrap_or(0))
                        .first::<SearchUser>(c)?;
                        list.push(friend);
                    } else {
                        let friend: SearchUser = users::table.select((
                            users::id,
                            users::name,
                            users::about,
                            users::avatar,
                            users::followerid
                        )).find(request.userid1.unwrap_or(0))
                        .first::<SearchUser>(c)?;
                        list.push(friend);
                    }
                }
                Ok(list)
            },
            Err(_) =>Err(diesel::result::Error::BrokenTransactionManager) 
        }
    }

}