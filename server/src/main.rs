use web_framework_lib::web::server;

fn main() {
    // TODO: Create application context, which is a map of names mapped to constructors.
    server::start("7878");
}
