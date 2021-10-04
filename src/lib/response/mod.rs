mod types;

use types::Listing;

pub use types::RedditResponse;

pub type RedditListing<T> = RedditResponse<Listing<T>>;
