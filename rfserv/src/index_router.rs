/**
 * Created at 2021/8/1 11:43
 *
 * @author Liangcheng Juves
 */
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_INDEX_ROUTERES: Vec<&'static str> = vec!["/index.html", "/index",];
    pub static ref ROOT_ROUTER: &'static str = "/";
}

pub fn include_index_router(uri: &str) -> bool {
    for router in DEFAULT_INDEX_ROUTERES.iter() {
        if *router == uri {
            return true;
        }
    }
    false
}
