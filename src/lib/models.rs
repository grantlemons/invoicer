use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub profile_picture: Option<Vec<u8>>,
    pub password_hash: String,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub profile_picture: Option<&'a [u8]>,
    pub password_hash: &'a str,
}

impl User {
    pub fn new(conn: &mut PgConnection, username: &str, email: &str, password: &str) -> Self {
        use argon2::{
            password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
            Argon2,
        };

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Hashing function failed")
            .to_string();

        let new_user = NewUser {
            username,
            email,
            profile_picture: None,
            password_hash: &hash,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(Self::as_returning())
            .get_result(conn)
            .expect("Error creating new user")
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(invoice_id))]
#[diesel(table_name = invoices)]
pub struct Invoice {
    pub invoice_id: i64,
    pub owner_id: i64,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = invoices)]
pub struct NewInvoice {
    pub owner_id: i64,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(proof_id))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_proof)]
pub struct InvoiceProof {
    pub proof_id: i64,
    pub invoice_id: i64,
    pub data: Vec<u8>,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_proof)]
pub struct NewInvoiceProof<'a> {
    pub invoice_id: i64,
    pub data: &'a [u8],
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(access_id))]
#[diesel(belongs_to(User, foreign_key = borrower_id))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_permissions)]
pub struct InvoicePermissions {
    pub access_id: i64,
    pub borrower_id: i64,
    pub invoice_id: i64,
    pub read_access: bool,
    pub write_access: bool,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_permissions)]
pub struct NewInvoicePermissions {
    pub borrower_id: i64,
    pub invoice_id: i64,
    pub read_access: bool,
    pub write_access: bool,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_line_items)]
pub struct InvoiceLineItem {
    id: i64,
    invoice_id: i64,
    item_name: String,
    item_price_usd: diesel::data_types::Cents,
}

#[derive(Insertable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Invoice))]
#[diesel(table_name = invoice_line_items)]
pub struct NewInvoiceLineItem {
    invoice_id: i64,
    item_name: String,
    item_price_usd: diesel::data_types::Cents,
}
