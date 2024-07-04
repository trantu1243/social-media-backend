use diesel::{result, PgConnection, QueryResult};
use diesel::QueryDsl;
use diesel::prelude::*;

use crate::schema::notifications;
use crate::{authorization::BearerToken, models::Notification, schema::users};

pub struct NotificationRespository;

impl NotificationRespository {
    pub fn get_notifications(c: &mut PgConnection, _auth: BearerToken)->QueryResult<Vec<Notification>>{
        let user = BearerToken::get_user(&_auth, c)?;

        let notification_ids: Option<Vec<Option<i32>>> = users::table.find(user.id)
        .select(users::notifications).get_result(c)?;
        let mut result: Vec<Notification> = Vec::new();
        let notifications = notification_ids.unwrap_or(Vec::new());
        for id in notifications {
            let notification: Notification = notifications::table.find(id.unwrap_or(0)).get_result(c)?;
            result.push(notification);
        }
        Ok(result)
    }
}