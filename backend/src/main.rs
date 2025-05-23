#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
struct Video {
    title: String,
    url: String,
}

#[get("/api/videos?<q>")]
fn search_videos(q: &str) -> Json<Vec<Video>> {
    let results = vec![
        Video {
            title: format!("{} 入门教程", q),
            url: "https://www.bilibili.com/video/abc123".to_string(),
        },
        Video {
            title: format!("{} 实战讲解", q),
            url: "https://www.bilibili.com/video/xyz456".to_string(),
        },
    ];
    Json(results)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![search_videos])
}
