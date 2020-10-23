#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use actix_web::{HttpServer, dev, web, error, http, middleware, App, Error, HttpMessage, HttpRequest,
                HttpResponse};
use log::info;
use std::process::Command;
use std::io::{self, Write};

extern crate simple_logger;

#[derive(Deserialize)]
pub struct Req {
    a: bool,
    b: bool,
    c: bool,
    d: f32,
    e: i32,
    f: i32
}

#[derive(Serialize, Deserialize)]
struct Res {
    k : f32,
    h : String
}

enum Kind {
    M, P, T, None
}

fn calc (h : &Kind, d: f32, e: i32, f : i32) -> f32
{
    match h {
        Kind::M => d + (d * e as f32 / 10.0),
        Kind::P => d + (d * (e - f) as f32/ 25.5),
        Kind::T => d - (d * f as f32 / 30.0),
        _ => 0.0
    }
}

fn exec_res(a : bool, b : bool, c : bool, d : f32, e : i32, f : i32) -> Option<Res>
{
    let mut k : f32 = 0.0;

    let expression_set = (a,b,c);

    let mut h: Kind = Kind::None;

    match expression_set {
        (a,b,c) if (a && b && !c) =>
            {
                h = Kind::M;
            },
        (a,b,c) if (a && b && c) =>
            {
                h = Kind::P;
            },
        (a,b,c) if (!a && b && c) =>
            {
                h = Kind::T;
            },
        _=> return None
    }

    k = calc(&h, d, e, f);

    if let Kind::P = h {
        k = 2.0 * d + (d * e as f32 / 100.0);
    }

    match expression_set {
        (a,b,c) if (a && b && !c) => {
            h = Kind::T;
            k = calc(&h, d, e, f);
        },
        (a,b,c) if (a && !b && c) => {
            h = Kind::M;
            k = calc(&h, d, e, f);
        },
        _=> {},
    }

    if let Kind::M = h {
        k = f as f32 + d + (d * e as f32 / 100.0);
    }

    let mut kind_text = String::from("");

    match h {
        Kind::M => kind_text = String::from("M"),
        Kind::P => kind_text = String::from("P"),
        Kind::T => kind_text = String::from("T"),
        _ => ()
    }

    Some(Res{k: k, h:kind_text})
}

fn exec(a : bool, b : bool, c : bool, d : f32, e : i32, f : i32) -> Option<f32>
{
    let mut k : f32 = 0.0;

    let expression_set = (a,b,c);

    let mut h: Kind = Kind::None;

    match expression_set {
        (a,b,c) if (a && b && !c) =>
            {
                h = Kind::M;
            },
        (a,b,c) if (a && b && c) =>
            {
                h = Kind::P;
            },
        (a,b,c) if (!a && b && c) =>
            {
                h = Kind::T;
            },
        _=> return None
    }

    k = calc(&h, d, e, f);

    if let Kind::P = h {
        k = 2.0 * d + (d * e as f32 / 100.0);
    }

    match expression_set {
        (a,b,c) if (a && b && !c) => {
            h = Kind::T;
            k = calc(&h, d, e, f);
        },
        (a,b,c) if (a && !b && c) => {
            h = Kind::M;
            k = calc(&h, d, e, f);
        },
        _=> {},
    }

    if let Kind::M = h {
        k = f as f32 + d + (d * e as f32 / 100.0);
    }

    let mut kind_text = String::from("");

    match h {
        Kind::M => kind_text = String::from("M"),
        Kind::P => kind_text = String::from("P"),
        Kind::T => kind_text = String::from("T"),
        _ => ()
    }

    Some(k)
}

pub async fn calculate(req: web::Json<Req>) -> Result<HttpResponse, error::Error>
{
    let c = exec_res(req.a, req.b, req.c, req.d, req.e, req.f);
    if c.is_some() {
        Ok(HttpResponse::Ok().json(c))
    }
    else {
        Ok(HttpResponse::Ok().body("Error"))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let srv = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/api")
                .service(web::resource("/calc").route(web::post().to(calculate)))
            )
        })
        .keep_alive(Some(0))
        .bind("127.0.0.1:8080")?
        .run();

    /*
    let a : bool = true;
    let b : bool = true;
    let c : bool = true;
    let d : f32 = 0.0;
    let e : i32 = 0;
    let f : i32 = 0;

    let k = exec(false, false, false, 30.0, 10, 10);
    match k {
        Some(k) => println!("k: {}", k),
        None => println!("error")
    }
    */
    srv.await
}

#[test]
fn test_a_b_c () {
    assert_eq!(exec(true, true, true, 30.0, 10, 10), Some(63.0));
}

#[test]
fn test_not_a_b_c () {
    assert_eq!(exec(false, true, true, 30.0, 10, 10), Some(20.0));
}

#[test]
fn test_a_not_b_c () {
    assert_eq!(exec(true, false, true, 30.0, 10, 10), None);
}

#[test]
fn test_a_b_not_c () {
    assert_eq!(exec(true, true, false, 30.0, 10, 10), Some(20.0));
}

#[test]
fn test_not_a_not_b_c () {
    assert_eq!(exec(false, false, true, 30.0, 10, 10), None);
}

#[test]
fn test_not_a_b_not_c () {
    assert_eq!(exec(false, true, false, 30.0, 10, 10), None);
}

#[test]
fn test_not_a_not_b_not_c () {
    assert_eq!(exec(false, false, false, 30.0, 10, 10), None);
}