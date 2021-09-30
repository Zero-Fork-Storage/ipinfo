pub(crate) mod fetchers;
pub(crate) mod getters;

pub fn kwabang_api(mode: &str, query: &str) -> String {
    let url = String::from("https://api.kwabang.net/");
    return format!("{}/{}/{}", url, mode, query);
}
