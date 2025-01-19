use actix_files::NamedFile;
use actix_web::{get, web,};



// Configure function for web routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(index)
    );
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("src/template/index.html")?)
}

