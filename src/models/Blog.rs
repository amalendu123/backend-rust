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

#[derive(Validate,Deserialize,Serialize)]
pub struct update_blog_url{
    pub uuid:String,
}


#[derive(Validate,Deserialize,Serialize)]
pub struct blog{
    pub uuid: String,
    pub Blog_title:String,
    pub author:String,
    pub content:String
}

impl blog {
    pub fn new(
        uuid:String,Blog_title:String,author:String,content:String
    )->blog{
        blog{
            uuid,
            Blog_title,
            author,
            content
        }
    }
}