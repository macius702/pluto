use std::collections::HashMap;

use ic_cdk::println;
use pluto::{
    http::{HttpRequest, HttpResponse, HttpServe},
    router::Router,
};
use serde_json::{json, value};

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use std::error::Error;


static WPISY: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));


fn dodaj_wpis(wpis: String) -> Result<(), Box<dyn Error>> {
    let mut wpisy = WPISY.lock().unwrap();
    wpisy.push(wpis);
    Ok(())
}

fn pobierz_wpisy() -> Vec<String> {
    ic_cdk::println!("[pobierz_wpisy] Entering function");
    ic_cdk::println!("[pobierz_wpisy] WPISY before lock: {:?}", *WPISY);
    let result = {
        let wpisy = WPISY.lock().unwrap();
        ic_cdk::println!("[pobierz_wpisy] WPISY after lock: {:?}", *wpisy);
        let clone = wpisy.clone();
        ic_cdk::println!("[pobierz_wpisy] WPISY after clone: {:?}", *wpisy);
        clone
    };
    ic_cdk::println!("[pobierz_wpisy] WPISY after unlock: {:?}", *WPISY);
    ic_cdk::println!("[pobierz_wpisy] Leaving function");
    result
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


    router.post("/dodaj_wpis/:value", true, |req: HttpRequest| async move {
        ic_cdk::println!("[POST /dodaj_wpis/:value] Entering handler");
        let received_body: Result<String, HttpResponse> = String::from_utf8(req.body)
            .map_err(|_| HttpServe::internal_server_error().unwrap_err());
        if let Some(val) = req.params.get("value") {
            ic_cdk::println!("[POST /dodaj_wpis/:value] Received value: {}", val);
            dodaj_wpis(val.to_string());
            ic_cdk::println!("[POST /dodaj_wpis/:value] Added entry: {}", val);
            let response = Ok(HttpResponse {
                status_code: 200,
                headers: HashMap::new(),
                body: json!({
                    "statusCode": 200,
                    "message": "Hello World from POST",
                    "paramValue": val.to_string(),
                    "receivedBody": received_body?,
                    "collection": pobierz_wpisy()
                })
                .into(),
            });
            ic_cdk::println!("[POST /dodaj_wpis/:value] Returning response: {:?}", response);
            response
        } else {
            // Handle the case when "value" is not present in the params
            let error_response = Err(HttpResponse {
                status_code: 400,
                headers: HashMap::new(),
                body: json!({
                    "statusCode": 400,
                    "message": "Bad Request",
                    "error": "Value not found in params"
                })
                .into(),
            });
            ic_cdk::println!("[POST /dodaj_wpis/:value] Returning error response: {:?}", error_response);
            error_response
        }    
    });
    router.get("/", false, |_req: HttpRequest| async move {
        ic_cdk::println!("[GET /] Entering handler");
        let response = Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from GET",
            })
            .into(),
        });
        ic_cdk::println!("[GET /] Returning response: {:?}", response);
        response
    });


    router.get("/pobierz_wpisy", false, |_req: HttpRequest| async move {
        ic_cdk::println!("[GET /pobierz_wpisy] Entering handler");
        let entries = pobierz_wpisy();
        ic_cdk::println!("[GET /pobierz_wpisy] Retrieved entries: {:?}", entries);
        let response = Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from GET /pobierz_wpisy",
                "body": entries
            })
            .into(),
        });
        ic_cdk::println!("[GET /pobierz_wpisy] Returning response: {:?}", response);
        response
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
