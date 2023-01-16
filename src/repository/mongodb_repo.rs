use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::todo_model::Todo;
use crate::models::user_model::User;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col1: Collection<User>,
    col2: Collection<Todo>,
}

impl MongoRepo {
        pub fn init() -> Self {
            dotenv().ok();
        let uri = match env::var("DATABASE_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("Rust");
        let col1: Collection<User> = db.collection("users");
        let col2: Collection<Todo> = db.collection("todos");
        MongoRepo { col1, col2 }
    }
    
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            password: new_user.password,
        };
        let user = self
            .col1
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
    

    
    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col1
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "title": new_user.password
                },
        };
        let updated_doc = self
            .col1
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col1
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        
        let cursors = self
            .col1
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn create_todo(&self, new_todo: Todo) -> Result<InsertOneResult, Error> {
        let new_doc = Todo {
            id: None,
            description: new_todo.description,
            title: new_todo.title,
            state: new_todo.state,
        };
        let todo = self
            .col2
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating todo");
        Ok(todo)
    }

    pub fn get_todo(&self, id: &String) -> Result<Todo, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let todo_detail = self
            .col2
            .find_one(filter, None)
            .ok()
            .expect("Error getting todo's detail");
        Ok(todo_detail.unwrap())
    }
    pub fn update_todo(&self, id: &String, new_todo: Todo) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_todo.id,
                    "description": new_todo.description,
                    "title": new_todo.title,
                    "state": new_todo.state,
                },
        };
        let updated_doc = self
            .col2
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating todo");
        Ok(updated_doc)
    }
    pub fn delete_todo(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let todo_detail = self
            .col2
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting todo");
        Ok(todo_detail)
    }
    pub fn get_all_todos(&self) -> Result<Vec<Todo>, Error> {
        let cursors = self
            .col2
            .find(None, None)
            .ok()
            .expect("Error getting list of todos");
        let todos = cursors.map(|doc| doc.unwrap()).collect();
        Ok(todos)
    }
}
