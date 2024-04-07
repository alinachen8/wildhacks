pub type UserId = i64;
pub type ListingId = i64;

#[derive(Clone)]
pub struct Listing {
    pub id: i64,
    pub name: String,
    pub goal: f64,
    pub interest: f64,
    pub image_url: String,
}

#[derive(Clone)]
pub struct DbUser {
    pub id: i64,
    pub name: String,
}

// should have a table with columns: user_id, listing_id, and number: float