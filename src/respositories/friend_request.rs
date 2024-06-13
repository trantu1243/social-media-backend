use crate::diesel::RunQueryDsl;
use crate::diesel::SelectableHelper;
use crate::models::FriendRequest;
use crate::models::NewFriendRequest;
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

}