#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
extern crate rand;

use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::{templates::Template, serve::StaticFiles};
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::RawStr;
use diesel::SqliteConnection;

mod link;
use link::{Link, LinkForm, LinkJson};

embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)>, links: Vec<Link> }


impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &DbConn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), links: Link::all(conn)}
    }

    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, links: Link::all(conn)}
    }
}


// HTML views
#[post("/", data = "<link_form>")]
fn form_add_link(link_form: Form<LinkForm>, conn: DbConn) -> Flash<Redirect> {
    let link = link_form.into_inner();
    if link.url.is_empty() {
        Flash::error(Redirect::to("/"), "URL cannot be empty.");
    }

    let (status, _alias) = Link::insert(link.url, &conn);

    if status {
        Flash::success(Redirect::to("/"), "Link successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Server error.")
    }
}


#[put("/<id>")]
fn form_change_link_status(id: i32, conn: DbConn) -> Result<Redirect, Template> {
    if Link::change_status_with_id(id, &conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't change the status.")))
    }
}


#[delete("/<id>")]
fn form_delete_link(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    if Link::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Link has been deleted."))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't delete the link.")))
    }
}


// Redirect feature
#[get("/<alias>")]
fn redirect(alias: &RawStr, conn: DbConn) -> Redirect {
    let link = Link::get_link_by_alias(alias, &conn);

    Redirect::to(link.url)
}


// API
#[get("/")]
fn index(msg: Option<FlashMessage>, conn: DbConn) -> Template {
    Template::render("index", &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
    })
}


#[get("/links", format = "json")]
fn api_get_links(conn: DbConn) -> Json<JsonValue> {
    let links = Link::all(&conn);
    Json(json!(links))
}


#[get("/links/<alias>", format = "json")]
fn api_get_link(alias: &RawStr, conn: DbConn) -> Json<JsonValue> {
    let link = Link::get_link_by_alias(alias, &conn);
    Json(json!(link))
}


#[post("/links", data = "<link>")]
fn api_post_link(link: Json<LinkJson>, conn: DbConn) -> Json<JsonValue> {
    if link.url.is_empty() {
        Json(json!({ "status": "Error" }));

    }

    let (status, alias) = Link::insert(link.0.url, &conn);

    if status {
        Json(json!({
            "status": "Successfully added a link",
            "link": format!("http://localhost:8000/redirect/{}", alias)
        }))
    } else {
        Json(json!({ "status": "Error" }))
    }
}


fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}


fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/redirect", routes![redirect])
        .mount("/api/v1", routes![api_get_links, api_get_link, api_post_link])
        .mount("/link", routes![form_add_link, form_change_link_status, form_delete_link])
        .attach(Template::fairing())
}


fn main() {
    rocket().launch();
}
