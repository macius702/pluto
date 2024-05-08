use std::collections::HashMap;

use ic_cdk::println;
use pluto::{
    http::{HttpRequest, HttpResponse, HttpServe},
    router::Router,
};
use serde_json::{json, value};


use std::cell::RefCell;

thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy| {
        let mut mutable_wpisy = wpisy.borrow_mut();
        mutable_wpisy.push(wpis);
    });
}

fn pobierz_wpisy() -> Vec<String>{
    WPISY.with(|wpisy| wpisy.borrow().clone())
}



pub(crate) fn setup() -> Router {

    dodaj_wpis("Pierwszy wpis".to_string());

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


    router.post("/dodaj_wpis/:value", false, |req: HttpRequest| async move {
        let received_body: Result<String, HttpResponse> = String::from_utf8(req.body)
            .map_err(|_| HttpServe::internal_server_error().unwrap_err());
        if let Some(val) = req.params.get("value") {
            dodaj_wpis(val.to_string());
            Ok(HttpResponse {
                status_code: 200,
                headers: HashMap::new(),
                body: json!({
                    "statusCode": 200,
                    "message": "Hello World from POST",
                    "paramValue": val.to_string(),
                    "receivedBody": received_body?
                })
                .into(),
            })
        } else {
            // Handle the case when "value" is not present in the params
            Err(HttpResponse {
                status_code: 400,
                headers: HashMap::new(),
                body: json!({
                    "statusCode": 400,
                    "message": "Bad Request",
                    "error": "Value not found in params"
                })
                .into(),
            }

            )

        }    
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


    router.get("/pobierz_wpisy", false, |_req: HttpRequest| async move {
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from GET /pobierz_wpisy",
                "body": pobierz_wpisy()
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
