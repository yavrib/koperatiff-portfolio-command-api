use connection::db::{establish_connection};
use diesel::{RunQueryDsl, ExpressionMethods, insert_into, update, delete};
use models::request::post::{Post};
use models::schema::posts::dsl::*;
use rocket_contrib::{Json, Value};

pub fn create_portfolio(post: Json<Post>) -> Json {
    let conn = establish_connection();
    let rows_inserted = insert_into(posts)
        .values(
            &post.clone()
        )
        .execute(&conn);

    match rows_inserted {
        Ok(1) => Json(json!({
            "successful": true,
            "message": "Portfolio inserted successfully."
        })),
        _ => Json(json!({
            "successful": false,
            "message": "An error occured!"
        }))
    }
}

pub fn update_portfolio(post: Json<Post>) -> Json {
    let conn = establish_connection();
    let rows_updated = update(posts)
        .filter(id.eq(&post.id.unwrap()))
        .set(
            &post.clone()
        )
        .execute(&conn);

    match rows_updated {
        Ok(1) => Json(json!({
            "successful": true,
            "message": "Portfolio inserted successfully."
        })),
        _ => Json(json!({
            "successful": false,
            "message": "An error occured!"
        }))
    }
}

pub fn remove_portfolio(post_id: i32) -> Json {
    let conn = establish_connection();
    let rows_deleted = delete(posts)
        .filter(id.eq(post_id))
        .execute(&conn);

    match rows_deleted {
        Ok(1) => Json(json!({
            "successful": true,
            "message": "Portfolio inserted successfully."
        })),
        _ => Json(json!({
            "successful": false,
            "message": "An error occured!"
        }))
    }
}
