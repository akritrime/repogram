use rocket::Rocket;
use rocket_contrib::{Json, Value};

use ::root_prefix;
use super::NewPost;

#[get("/")]
fn get_all() -> &'static str {
    "All posts"
}

#[get("/<id>")]
fn get_one(id: usize) -> String {
    format!("Post with id {}", id)
}

#[post("/", format = "application/json", data = "<new_post>")]
fn create_new_post(new_post: Json<NewPost>) -> Json<Value> {
    let new_post = new_post.0;
    println!("Title: {}\nContent: {}", new_post.title(), new_post.content());
    Json(json!({
        "status": "success",
        "data": new_post
    }))

}

pub fn routes(rkt: Rocket) -> Rocket {
    rkt.mount(&root_prefix("posts/"), routes![
        get_all,
        get_one,
        create_new_post
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn get_all_works() {
        assert_eq!(get_all(), "All posts")
    }

    #[test]
    fn get_one_works() {
        assert_eq!(get_one(20), "Post with id 20")
    }

    #[test]
    fn creat_new_post_works() {
        let new_post = NewPost {
            title: "Title".into(),
            content: "Content".into()
        };

        let np_json = json!(new_post);

        let res = create_new_post(Json(new_post)).0;
        assert_eq!(res["status"], "success");
        assert_eq!(res["data"], np_json);
    }
}