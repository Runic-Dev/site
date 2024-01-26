use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, handlebars::Handlebars};
use std::{path::{Path, PathBuf}, fs::ReadDir};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", &()) 
}

#[get("/test")]
fn test() -> Template {
    Template::render("test", &()) 
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", &()) 
}

#[get("/streaming")]
fn streaming() -> Template {
    Template::render("streaming", &()) 
}
#[get("/blog")]
fn blog() -> Template {
    Template::render("blog", &()) 
}
#[get("/contact")]
fn contact() -> Template {
    Template::render("contact", &()) 
}
#[launch]
fn rocket() -> _ {
    const PARTIALS: &[&'static str; 1] = &[
        "templates/partials"
    ];
    rocket::build()
        .attach(Template::custom(|engines| {
            let hbs: &mut Handlebars = &mut engines.handlebars;
            PARTIALS.iter().for_each(|partials_path| setup_subfolder_templates(hbs, partials_path));
        }))
        .mount("/public", FileServer::from("public"))
        .mount("/", routes![index, test, projects, streaming, blog, contact])
}

fn setup_subfolder_templates(hbs: &mut Handlebars, path: &str) {
    println!("Checking templates subfolder {}...", path);
    let templates_path = Path::new(path);
    let templates_dir = match templates_path.read_dir() {
        Ok(dir) => dir,
        Err(_) => {
            println!("Problem with templates directory: {:?}", path);
            return;
        }
    };

    process_templates(hbs, templates_dir);
}

fn process_templates(hbs: &mut Handlebars, templates_dir: ReadDir) {
    println!("Processing templates for {:?}", templates_dir);
    for entry in templates_dir.filter_map(Result::ok) {
        let template_path = entry.path();
        if let Some(template_name) = extract_template_name(&template_path) {
            register_template(hbs, &template_name, &template_path);
        }
    }
}

fn extract_template_name(path: &PathBuf) -> Option<String> {
    println!("Extracting template name for {:?}...", path);
    let full_filename = path.file_stem()?.to_str()?;
    full_filename.find('.').map(|index| &full_filename[..index])
        .map_or_else(|| None, |f| Some(f.to_string()))
}

fn register_template(hbs: &mut Handlebars, name: &str, path: &PathBuf) {
    println!("Registering template {} at path {:?}", name, path);
    if let Err(err) = hbs.register_template_file(name, path) {
        println!("Problem processing template {}: {:?}", name, err);
    }
}
