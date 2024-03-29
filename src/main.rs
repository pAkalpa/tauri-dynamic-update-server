mod models;
mod logics;

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use crate::models::{Configs, UpdateContent};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use crate::logics::{get_release_data};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> Redirect {
    let conf = match envy::from_env::<Configs>() {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", e);
            panic!()
        }
    };
    Redirect::permanent(conf.home_redirect_url)
}

#[get("/<target>?<version>&<arch>")]
async fn get_update_data(target: &str, version: &str, arch: &str) -> Result<Json<UpdateContent>, Status> {
    return match target {
        "windows" | "linux" | "darwin" => {
            match get_release_data(version, arch, target).await {
                Some(data) => Ok(data),
                None => Err(Status::NoContent)
            }
        },
        _ => Err(Status::NoContent) }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, get_update_data])
}