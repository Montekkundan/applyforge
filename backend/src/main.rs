mod models;
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::thread_rng;

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "Hello from Actix Web!"}))
}

#[post("/api/register")]
async fn register(
    db: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> impl Responder {
    let user_exists: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = $1")
        .bind(&req.username)
        .fetch_one(db.get_ref())
        .await
    {
        Ok(count) => count,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if user_exists.0 > 0 {
        return HttpResponse::BadRequest().json(AuthResponse {
            success: false,
            message: "Username already exists".into(),
        });
    }
    // Hash the password
    let salt = SaltString::generate(&mut thread_rng());
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let _ = sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(&req.username)
        .bind(&password_hash)
        .execute(db.get_ref())
        .await;
    HttpResponse::Ok().json(AuthResponse {
        success: true,
        message: "Registered successfully".into(),
    })
}

#[post("/api/login")]
async fn login(
    db: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> impl Responder {
    // Fetch the password hash for the user
    let row = sqlx::query!("SELECT password FROM users WHERE username = $1", req.username)
        .fetch_optional(db.get_ref())
        .await
        .ok()
        .flatten();
    if let Some(record) = row {
        let parsed_hash = PasswordHash::new(&record.password).unwrap();
        if Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            return HttpResponse::Ok().json(AuthResponse {
                success: true,
                message: "Login successful".into(),
            });
        }
    }
    HttpResponse::Unauthorized().json(AuthResponse {
        success: false,
        message: "Invalid credentials".into(),
    })
}

#[post("/api/jobs")]
async fn add_job(
    db: web::Data<PgPool>,
    job: web::Json<JobApplication>,
) -> impl Responder {
    let job = job.into_inner();
    let rec = sqlx::query!(
        "INSERT INTO jobs (company, position, status, date, username) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        job.company,
        job.position,
        format!("{:?}", job.status),
        job.date,
        job.username
    )
    .fetch_one(db.get_ref())
    .await;
    match rec {
        Ok(record) => {
            let mut job = job;
            job.id = record.id as usize;
            HttpResponse::Ok().json(job)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/api/jobs/{username}")]
async fn get_jobs(
    db: web::Data<PgPool>,
    username: web::Path<String>,
) -> impl Responder {
    let jobs = sqlx::query!(
        "SELECT id, company, position, status, date, username FROM jobs WHERE username = $1",
        username.as_str()
    )
    .fetch_all(db.get_ref())
    .await;
    match jobs {
        Ok(records) => {
            let jobs: Vec<models::JobApplication> = records.into_iter().map(|rec| models::JobApplication {
                id: rec.id as usize,
                company: rec.company,
                position: rec.position,
                status: match rec.status.as_str() {
                    "Applied" => models::ApplicationStatus::Applied,
                    "Interview" => models::ApplicationStatus::Interview,
                    "Offer" => models::ApplicationStatus::Offer,
                    "Rejected" => models::ApplicationStatus::Rejected,
                    "Accepted" => models::ApplicationStatus::Accepted,
                    _ => models::ApplicationStatus::Applied,
                },
                date: rec.date,
                username: rec.username,
            }).collect();
            HttpResponse::Ok().json(jobs)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    // Ensure users table exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    // Ensure jobs table exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS jobs (
            id SERIAL PRIMARY KEY,
            company TEXT NOT NULL,
            position TEXT NOT NULL,
            status TEXT NOT NULL,
            date TEXT NOT NULL,
            username TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create jobs table");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(hello)
            .service(register)
            .service(login)
            .service(add_job)
            .service(get_jobs)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}