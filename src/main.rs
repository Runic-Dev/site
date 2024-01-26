use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", &()) 
}

#[get("/test")]
fn test() -> Template {
    Template::render("test", &()) 
}


#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing())
        .mount("/public", FileServer::from("public"))
        .mount("/", routes![index, test])
}
