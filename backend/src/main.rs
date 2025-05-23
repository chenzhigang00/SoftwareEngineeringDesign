#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
struct Video {
    title: String,
    url: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    code: u8,
    message: String,
    data: T,
}

#[get("/api/videos?<keyword>")]
fn search_videos(keyword: &str) -> Json<ApiResponse<Vec<Video>>> {
    // 实际项目中可根据 keyword 查询数据库
    let results = match keyword {
        "动态规划" => vec![
            Video {
                title: "动态规划".to_string(),
                url: "https://billbill/dp_learning".to_string(),
            },
            Video {
                title: "递归".to_string(),
                url: "https://billbill/digui_learning".to_string(),
            },
        ],
        _ => vec![],
    };
    Json(ApiResponse {
        code: 0,
        message: "操作成功".to_string(),
        data: results,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![search_videos])
}