use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;


fn main() {
    println!("Hello, world!");
}
