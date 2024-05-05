use std::collections::HashMap;

use ic_cdk::println;
use pluto::{
    http::{HttpRequest, HttpResponse, HttpServe},
    router::Router,
};
use serde_json::json;

pub(crate) fn setup() -> Router {
    let mut router = Router::new();

    router.put("/:value", false, |req: HttpRequest| async move {
        println!("Hello World from PUT {:?}", req.params.get("value"));

        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from PUT",
                "paramValue": req.params.get("value")
            })
            .into(),
        })
    });
    router.post("/", false, |req: HttpRequest| async move {
        let received_body: Result<String, HttpResponse> = String::from_utf8(req.body)
            .map_err(|_| HttpServe::internal_server_error().unwrap_err());
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from POST",
                "receivedBody": received_body?
            })
            .into(),
        })
    });
    router.get("/", false, |_req: HttpRequest| async move {
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from GET",
            })
            .into(),
        })
    });

    // Imlpement get for retrieving index.html
    router.get("/index.html", false, |_req: HttpRequest| async move {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
    
        Ok(HttpResponse {
            status_code: 200,
            headers,
            body: "<!DOCTYPE html>
            <html>
            <head>
                <title>Sample Page</title>
            </head>
            <body>
                <h1>Hello, World!</h1>
            </body>
            </html>".into(), 
               })
    });

    // get for retrieving sample json
    router.get("/sample.json", false, |_req: HttpRequest| async move {
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
    "data": {
        "users": [
            {
                "id": 1,
                "name": "John Doe",
                "email": "john.doe@example.com"
            },
            {
                "id": 2,
                "name": "Jane Doe",
                "email": "jane.doe@example.com"
            }
        ],
        "pagination": {
            "current_page": 1,
            "total_pages": 10
        }
    }
})
            .into(),
        })
    });

    router
}
