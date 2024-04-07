pub type UserId = i64;
pub type ListingId = i64;

#[derive(Clone)]
pub struct Listing {
    listing_id_: i64,
    name_: String,
    price_: f64,
    interest_: f64,
    image_url_: String,
}

#[derive(Clone)]
pub struct DbUser {
    user_id_: i64,
    name_: String,
}

// should have a table with columns: user_id, listing_id, and number: float