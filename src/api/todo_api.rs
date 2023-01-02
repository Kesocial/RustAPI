use crate::{models::todo_model::Todo, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/todo", data = "<new_todo>")]
pub fn create_todo(
    db: &State<MongoRepo>,
    new_todo: Json<Todo>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Todo {
        id: None,
        description: new_todo.description.to_owned(),
        title: new_todo.title.to_owned(),
        state: new_todo.state.to_owned(),
    };
    let todo_detail = db.create_todo(data);
    match todo_detail {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/todo/<path>")]
pub fn get_todo(db: &State<MongoRepo>, path: String) -> Result<Json<Todo>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let todo_detail = db.get_todo(&id);
    match todo_detail {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/todo/<path>", data = "<new_todo>")]
pub fn update_todo(
    db: &State<MongoRepo>,
    path: String,
    new_todo: Json<Todo>,
) -> Result<Json<Todo>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Todo {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        description: new_todo.description.to_owned(),
        title: new_todo.title.to_owned(),
        state: new_todo.state.to_owned(),
    };
    let update_result = db.update_todo(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_todo_info = db.get_todo(&id);
                return match updated_todo_info {
                    Ok(todo) => Ok(Json(todo)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
#[delete("/todo/<path>")]
pub fn delete_todo(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_todo(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Todo successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/todos")]
pub fn get_all_todos(db: &State<MongoRepo>) -> Result<Json<Vec<Todo>>, Status> {
    let todos = db.get_all_todos();
    match todos {
        Ok(todos) => Ok(Json(todos)),
        Err(_) => Err(Status::InternalServerError),
    }
}
