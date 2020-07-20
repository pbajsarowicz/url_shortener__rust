use diesel::{self, prelude::*};
use rand::{self, Rng};

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";


mod schema {
    table! {
        links {
            id -> Nullable<Integer>,
            url -> Text,
            alias -> Text,
            is_active -> Bool,
        }
    }
}

use self::schema::links;
use self::schema::links::dsl::{links as all_links, is_active as links_active};


#[table_name="links"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Link {
    pub id: Option<i32>,
    pub url: String,
    pub alias: String,
    pub is_active: bool
}


#[derive(FromForm)]
pub struct LinkForm {
    pub url: String
}


#[derive(Serialize, Deserialize)]
pub struct LinkJson {
    pub url: String
}


fn generate_alias() -> String {
    let mut id = String::with_capacity(5);
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        id.push(BASE62[rng.gen::<usize>() % 62] as char);
    }

    return id
}


impl Link {
    pub fn all(conn: &SqliteConnection) -> Vec<Link> {
        all_links.order(links::id.desc()).load::<Link>(conn).unwrap()
    }

    pub fn insert(url: String, conn: &SqliteConnection) -> (bool, String) {
        let alias = generate_alias();
        let link = Link { id: None, url: url, alias: alias.to_string(), is_active: true };
        let status = diesel::insert_into(links::table).values(&link).execute(conn).is_ok();

        return (status, alias);
    }

    pub fn get_link_by_alias(alias: &str, conn: &SqliteConnection) -> Link {
        let link = all_links
            .filter(links::alias.eq(alias))
            .first::<Link>(conn)
            .expect("Error while querying for links");

        return link
    }

    pub fn change_status_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let task = all_links.find(id).get_result::<Link>(conn);
        if task.is_err() {
            return false;
        }

        let new_status = !task.unwrap().is_active;
        let updated_task = diesel::update(all_links.find(id));
        updated_task.set(links_active.eq(new_status)).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_links.find(id)).execute(conn).is_ok()
    }
}
