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


#[get("/<base>/<num>", rank = 1)]
fn rekursion(base: u32, num: &str) -> Result<Template, Redirect> {
    if base < 2 || base > 36 {
        return Err(Redirect::to(uri!("/")))
    }

    match u32::from_str_radix(num, base) {
        Ok(n) => {
            let level = format_radix(n + 1, base);
            Ok(Template::render("index", context! { level, base }))
        }
        Err(_) => Err(Redirect::to(uri!("/")))
    }
}

#[get("/<_..>" , rank = 2)]
fn index() -> Redirect {
    Redirect::to(uri!("/2/0"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, rekursion])
        .mount("/", FileServer::from(relative!("/site/static")).rank(0))
        .attach(Template::fairing())
}