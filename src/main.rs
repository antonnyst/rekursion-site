use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};
use rocket::response::Redirect;
#[macro_use] extern crate rocket;

// https://stackoverflow.com/a/50278316
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}


#[get("/<base>/<num>")]
fn rekursion(base: u32, num: &str) -> Option<Template> {
    if base < 2 || base > 36 {
        return None
    }

    match u32::from_str_radix(num, base) {
        Ok(n) => {
            let level = format_radix(n + 1, base);
            Some(Template::render("index", context! { level, base }))
        }
        Err(_) => None
    }
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/2/0"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![rekursion])
        .mount("/", FileServer::from(relative!("/site/static")))
        .attach(Template::fairing())
}