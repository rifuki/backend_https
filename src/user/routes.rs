use actix_web::web;

use crate::user::handler::{
    get_all_users,
    get_user,
    update_user,
    delete_user
};

pub fn scoped_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_all_users))
            .service(
                web::resource("/{id_user}")
                    .get(get_user)
                    .put(update_user)
                    .delete(delete_user)
            )
    );
}