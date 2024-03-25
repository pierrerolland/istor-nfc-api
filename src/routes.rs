use warp::Filter;
use crate::handlers::AssignStoryBody;
use super::handlers;


pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_unassigned_stories().or(assign_story())
}

fn get_unassigned_stories() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("unassigned-stories").and(warp::get()).and_then(handlers::get_unassigned_stories)
}

fn assign_story() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("assign-story").and(warp::post()).and(warp::body::json()).and_then(|body: AssignStoryBody| handlers::assign_story(body))
}
