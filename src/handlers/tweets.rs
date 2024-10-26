use actix_web::{get, Responder};

#[get("/bah")]
pub async fn bah() -> impl Responder {
    let mut result = String::from("ba");
    let num_h = rand::random::<u8>() % 15 + 4;
    result.extend(std::iter::repeat('h').take(num_h as usize));
    format!("{}", result)
}
