use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use uuid::Uuid;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::error::Error;
use toml;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct Pipeline {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub positions: HashMap<Uuid, (f32, f32)>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Connection {
    pub from_node_id: Uuid,
    pub to_node_id: Uuid,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub content: String,
}

pub fn save_pipeline(pipeline: &Pipeline, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let toml_str = toml::to_string_pretty(pipeline)?;
    let mut file = fs::File::create(path)?;
    file.write_all(toml_str.as_bytes())?;
    Ok(())
}

pub fn load_pipeline(path: &PathBuf) -> Result<Pipeline, Box<dyn Error>> {
    if !path.exists() {
        let default_pipeline = Pipeline {
            nodes: Vec::new(),
            connections: Vec::new(),
            positions: HashMap::new(),
        };
        save_pipeline(&default_pipeline, path)?;
        return Ok(default_pipeline);
    }
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let pipeline: Pipeline = toml::from_str(&contents)?;
    Ok(pipeline)
}
#[utoipa::path(
    get,
    path = "/pipeline",
    responses(
        (status = 200, description = "Get pipeline", body = Pipeline)
    )
)]
#[get("/pipeline")]
pub async fn get_pipeline_handler() -> impl Responder {
    let path = PathBuf::from("pipeline.toml");
    match load_pipeline(&path) {
        Ok(pipeline) => HttpResponse::Ok().json(pipeline),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/pipeline",
    request_body = Pipeline,
    responses(
        (status = 200, description = "Pipeline updated successfully")
    )
)]
#[post("/pipeline")]
pub async fn update_pipeline_handler(new_pipeline: web::Json<Pipeline>) -> impl Responder {
    let path = PathBuf::from("pipeline.toml");
    match save_pipeline(&new_pipeline.into_inner(), &path) {
        Ok(_) => HttpResponse::Ok().body("Pipeline has been updated!"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
