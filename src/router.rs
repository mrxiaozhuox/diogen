pub fn url_to_route(url: &str) -> String {
    let url = url.to_string();

    let list = url.split("/#/").collect::<Vec<&str>>();

    if list.len() <= 1 {
        return String::from("/");
    }

    format!("/{}", list.get(1).unwrap())
}
