use actix_files::NamedFile;
use actix_web::{get, web};


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/assets")
            .service(htmx)
            .service(tailwind)
            .service(profile)
    );
}

// There have to be a better way of doing this (Don't wanna use nginx ATM) and don't want to use regex
#[get("/htmx.min.js")]  // Changed from "htmx.min.js" to "/assets/htmx.min.js"
async fn htmx() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("src/template/assets/htmx.min.js")?)
}

#[get("/tailwind.min.js")]  // Changed from "tailwind.min.js" to "/assets/tailwind.min.js"
async fn tailwind() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("src/template/assets/tailwind.min.js")?)
}

#[get("/profile.jpeg")]  // Hero profile picture
async fn profile() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("src/template/assets/images/PeterKnausProfile.jpeg")?)
}
