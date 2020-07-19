mod list;

use warp::Filter;

pub fn endpoints() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list::endpoint()
}
