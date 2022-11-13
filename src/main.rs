use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};
use rocket::response::Redirect;
#[macro_use] extern crate rocket;


#[get("/<num>")]
fn rekursion(num: usize) -> Template {
    let level = num + 1;
    Template::render("index", context! { level })
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/1"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![rekursion])
        .mount("/", FileServer::from(relative!("/site/static")))
        .attach(Template::fairing())
}