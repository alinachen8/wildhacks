pub type UserId = i32;
pub type ListingId = i32;


#[derive(Clone)]
pub struct Listing {
    listing_id_: i32,
    name_: String,
    price_: f64,
    interest_: f64,
    image_url_: String,
}

#[derive(Clone)]
pub struct DbUser {
    user_id_: i32,
    name_: String,
}

// should have a table with columns: user_id, listing_id, and number: float