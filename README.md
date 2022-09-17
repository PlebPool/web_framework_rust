# web_framework_rust
Web framework written in pure rust. Will also semi-build-in a DI framework. For my high school diploma.
![image](https://user-images.githubusercontent.com/64704277/190867762-dd4165a5-0225-49f1-a296-f9d1daca9308.png)
# EXAMPLE USAGE
```
pub fn index(transaction: &mut Transaction) {
    let res: &mut Response = transaction.res_mut();
    res.set_status(200)
        .set_reason_phrase("OK")
        .set_body_to_file("/html/index.html") // This will set response body to file src/public/html/index.html
            .expect("Failed to read file");
    // The transaction gets resolved after this function completes.
}

pub fn path_param_test(transaction: &mut Transaction) {
    // Here, we're extracting a path cell by index. (url decoded).
    let path_cell: String = transaction.req()
        .request_line_data()
        .get_path_cell_by_index_url_decoded(1)
        .unwrap();
    let res: &mut Response = transaction.res_mut();
    res.set_status(200)
        .set_reason_phrase("OK")
        .set_body(path_cell);
    // The transaction gets resolved after this function completes.
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    let mut container: IocContainer = IocContainer::default();
    let mut rhc: RouteHandlerContainer = RouteHandlerContainer::new();
    rhc.insert("/", index, HttpMethod::GET);
    // {whatever} = a path variable. Strings we insert as keys get translated to regex. The contents between {} are purely semantic atm.
    rhc.insert("/hey/{a}/hey", path_param_test, HttpMethod::GET);
    container.install_reference_provider(Arc::new(rhc));
    server::start("7878", Arc::new(container));
}
```
