use reqwest::blocking::{Client, Request};
use serde_json::json;

use crate::GRAPHQL_ENDPOINT;

pub fn get_rest_requests() {
    let _queries: &[&str] = &[
        "http://localhost:8080/products?min=1.0&max=50.0",
        "http://localhost:8080/products",
        "http://localhost:8080/orders",
        "http://localhost:8080/customers",
    ];
}

pub fn get_graphql_queries() -> Vec<String> {
    let mut requests = Vec::new();

    requests.push(
        r#"
        query {
            products(priceFilter: { min: 10, max: 100 }) {
                id
                name
                price
            }
        }
        "#.into()
    );

    requests.push(
        r#"
        query allProducts {
            products {
                id
                name
                price
                description
            }
        }
        "#.into()
    );

    requests.push(
        r#"
        query allProducts {
            products {
                id
                name
                price
                description
            }
        }
        "#.into()
    );

    requests
}

pub fn build_graphql_request(client: &Client, query: &str) -> Request {
    let body = json!({"query": query});

    client
        .post(GRAPHQL_ENDPOINT)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .build()
        .unwrap()
}
