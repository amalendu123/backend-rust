use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::Blog::blog;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("blog").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("blog"),
        })
    }

    pub async fn get_all_blogs(&self) -> Option<Vec<blog>> {
        let result = self.client.select("blog").await;
        match result {
            Ok(all_blog) => Some(all_blog),
            Err(_) => None,
        }
    }

    pub async  fn add_blog(&self,new_blog:blog) -> Option<blog> {
        let create_blog = self.client
                                .create(("blog",new_blog.uuid.clone()))
                                .content(new_blog)
                                .await;
                            match create_blog{
                                Ok(created)=>created,
                                Err(_)=>None
                            }
    }
    pub async fn update_blog(&self, uuid: String) -> Result<Option<blog>, Error> {
        let find_blog:Result<Option<blog>, Error> = self.client.select(("blog", &uuid)).await;
    
        match find_blog {
            Ok(found) => match found {
                Some(found_blog) => {
                    let update_blog = self
                        .client
                        .update(("blog", &uuid))
                        .merge(blog {
                            uuid: uuid.clone(),
                            Blog_title: found_blog.Blog_title.clone(),
                            content: String::from("Sold"),
                            author: found_blog.author.clone(),
                        })
                        .await;
    
                    match update_blog {
                        Ok(updated_vec) => {
                            // Assuming `updated_vec` is a Vec<blog>
                            if let Some(updated_blog) = updated_vec.into_iter().next() {
                                Ok(Some(updated_blog))
                            } else {
                                Ok(None)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
    
}
