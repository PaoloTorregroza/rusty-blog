use rocket_db_pools::Connection;

use crate::{
    api::SignupForm,
    db::{self, BlogDatabase},
};

pub async fn register_user(mut db: Connection<BlogDatabase>, form_user: SignupForm) -> bool {
    if form_user.conf_password != form_user.password {
        return false;
    }

    let new_user = db::insert_user(
        db,
        form_user.name,
        form_user.email,
        form_user.password,
        None,
    )
    .await;

    println!("{:?}", new_user);

    new_user.is_some()
}
