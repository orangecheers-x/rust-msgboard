use chrono::NaiveDateTime;

pub struct UserUUID {
    pub user_id: i32,
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
