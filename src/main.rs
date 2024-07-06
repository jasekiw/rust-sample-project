#[macro_use]
extern crate rocket;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;
use sqlx::{Error};
use chrono::{Utc};

#[derive(Database)]
#[database("RustDatabase")]
struct RustDatabase(sqlx::MySqlPool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/scale")]
async fn scale_route(mut db: Connection<RustDatabase>) -> String {
    // let tx = db.begin().await.expect("Failed to begin transaction");
    let rest_number = get_number().await.ok().unwrap();

    sqlx::query("INSERT INTO random_numbers (Number) VALUES (?)")
        .bind(rest_number)
        .execute(&mut **db).await.expect("Failed to insert");

    let mut current_scale = 3;
    let current_scale_rows: Result<(i32,), Error> = sqlx::query_as("SELECT ScaleNumber FROM current_scale ORDER BY CreatedAt DESC LIMIT 1")
        .fetch_one(&mut **db).await;

    match current_scale_rows {
        Ok(value) => {
            current_scale = value.0;
        }
        Err(_) => {}
    }

    let row: (i32,) = sqlx::query_as("SELECT CAST(ROUND(AVG(Number)) AS INT) FROM random_numbers")
        .fetch_one(&mut **db).await.expect("Failed to get number from db");
    let db_number = row.0;



    let new_scale = scale(rest_number, db_number, current_scale);

    let now = Utc::now();
    sqlx::query("INSERT INTO current_scale (ScaleNumber, CreatedAt) VALUES (?, ?)")
        .bind(new_scale)
        .bind(now)
        .execute(&mut **db).await.expect("Failed to insert");
    // tx.commit().await.expect("Failed to commit transaction");
    new_scale.to_string()
}


fn scale(rest_result: i32, db_result: i32, current_scale: i32) -> i32 {
    return if rest_result + db_result > 100 {
        current_scale + 10
    } else {
        let mut new_scale = current_scale - 10;
        if new_scale < 3 {
            new_scale = 3;
        }
        new_scale
    };
}


// todo: have a more generic error response here
async fn get_number() -> Result<i32, reqwest::Error> {
    let resp = reqwest::get("https://www.randomnumberapi.com/api/v1.0/random?min=1&max=100")
        .await?
        .json::<Vec<i32>>()
        .await?;

    let value = resp.get(0).cloned().unwrap();
    return Ok(value);
}

#[launch]
async fn rocket() -> _ {
    // let pool = MySqlPoolOptions::new()
    //        .max_connections(5)
    //        .connect("mysql://root:@localhost/RustDatabase").await?;

    // // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    // let row: (i64,) = sqlx::query_as("SELECT ROUND(AVG(Number)) FROM random_numbers")
    //     .fetch_one(&pool).await?;

    rocket::build()
        .attach(RustDatabase::init())
        // .manage(pool)
        .mount("/", routes![index])
        .mount("/", routes![scale_route])
}
