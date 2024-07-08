use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rocket_db_pools::sqlx::{self, database, Result, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("rusty_blog")]
pub struct BlogDatabase(sqlx::PgPool);

// MODELS
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

// CRUD ACTIONS
pub async fn insert_user(
    mut db: Connection<BlogDatabase>,
    name: String,
    email: String,
    password: String,
    is_admin: Option<bool>,
) -> Option<User> {
    let hashed = encrypt_password(password);

    if hashed.is_empty() {
        return None
    }

    match sqlx::query(
        r#"INSERT INTO users (name, email, "password", is_admin) VALUES (?,?,?,?) RETURNING id, name, email, "password", is_admin;"#,
    )
    .bind(name)
    .bind(email)
    .bind(hashed)
    .bind(is_admin.unwrap_or(false))
    .fetch_one(&mut **db)
    .await
    {
        Ok(result) => {
            return Some(
                User { 
                    id: result.get("id"), 
                    name: result.get("name"), 
                    email: result.get("email"), 
                    password: result.get("password"), 
                    is_admin: result.get("is_admin") 
                }
            );
        },
        Err(err) => panic!("{}",err),
    };

    return None;
}


// PASSWORD ENCRYPT USING ARGON2
pub fn encrypt_password(password: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let result = argon2.hash_password(password.as_bytes(), &salt);
    match result {
        Ok(hashed) => return hashed.to_string(),
        Err(err) => {
            eprintln!("{}", err); 
            return "".to_string();
        },
    }
}