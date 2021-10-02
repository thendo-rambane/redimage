mod types;

use types::{Listing, RedditResponse};

pub type RedditListing<T> = RedditResponse<Listing<T>>;
