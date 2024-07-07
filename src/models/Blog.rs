//User

use serde::{Deserialize,Serialize};
use validator::Validate;

#[derive(Validate,Deserialize,Serialize)]
pub struct getBlogRequest {
    #[validate(length(min = 1,message = "A title is required"))]
    pub Blog_title : String,
    #[validate(length(min = 1, message = "An author is required"))]
    pub author: String,
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
}