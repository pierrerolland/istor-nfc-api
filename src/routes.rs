use warp::Filter;
use warp::http::{HeaderMap, HeaderValue};
use crate::handlers::AssignStoryBody;
use super::handlers;


pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let mut headers = HeaderMap::new();
    headers.insert("Content-type", HeaderValue::from_static("application/json;charset=UTF-8"));

    get_unassigned_stories().or(assign_story()).with(warp::reply::with::headers(headers))
}

fn get_unassigned_stories() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("unassigned-stories").and(warp::get()).and_then(handlers::get_unassigned_stories)
}

fn assign_story() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("assign-story").and(warp::post()).and(warp::body::json()).and_then(|body: AssignStoryBody| handlers::assign_story(body))
}
