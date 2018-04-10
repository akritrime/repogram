use rocket::{Rocket};

#[get("/")]
fn index() -> &'static str {
    "All posts"
}

pub fn routes(rkt: Rocket) -> Rocket {
    rkt.mount("posts/", routes![index])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn index_works() {
        assert_eq!(index(), "All posts")
    }
}