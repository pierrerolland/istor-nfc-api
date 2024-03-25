use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::rename;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AssignStoryBody {
    card_id: String,
    story_title: String
}

pub async fn get_unassigned_stories() -> Result<impl warp::Reply, warp::Rejection> {
    let mut list = Vec::new();
    let files = fs::read_dir(std::env::var("ISTOR_STORIES_DIRECTORY").expect("ISTOR_STORIES_DIRECTORY must be defined")).unwrap();

    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("mp3") {
            if let Some(file_name) = path.file_stem().and_then(std::ffi::OsStr::to_str) {
                if !file_name.contains('¤') {
                    list.push(file_name.to_string());
                }
            }
        }
    }

    Ok(warp::reply::json(&list))
}

pub async fn assign_story(body: AssignStoryBody) -> Result<impl warp::Reply, warp::Rejection> {
    let dir = std::env::var("ISTOR_STORIES_DIRECTORY").expect("ISTOR_STORIES_DIRECTORY must be defined");
    let files = fs::read_dir(&dir).unwrap();

    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("mp3") {
            if let Some(file_name) = path.file_stem().and_then(std::ffi::OsStr::to_str) {
                if file_name.eq(&body.story_title) {
                    rename(
                        path.to_str().unwrap(),
                        format!("{}/{}¤{}.mp3", &dir, &body.card_id, &body.story_title)
                    ).unwrap();
                }
            }
        }
    }

    Ok(warp::reply::reply())
}
