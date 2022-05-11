use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;

use serde::{Deserialize, Serialize};
use serde_yaml::{self};

// TODO move definitions to separate file(s)
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    title: String,
    subtitle: String,
    logo: String,
    icon: String,
    header: bool,
    theme: String,
    groups: Vec<Group>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    name: String,
    items: Vec<Item>,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    icon: String,
    subtitle: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    name: String,
    icon: String,
    url: String,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate<'a> {
    site_config: &'a Config,
}

async fn index() -> Result<HttpResponse> {
    // TODO make the config file be an argument at runtime
    let f = std::fs::File::open("config.yaml").expect("Could not open file.");
    let site_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    // TODO remove
    println!("{:?}", site_config);

    let s = DashboardTemplate {
        site_config: &site_config,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO let this be user-defined
    let port = 8080;

    // start http server
    println!("starting server on port {}", port);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
