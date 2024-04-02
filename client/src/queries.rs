use reqwest::{blocking::{Client, Request}, Method};
use serde_json::json;

use crate::{GRAPHQL_ENDPOINT, HOST_NAME};

pub fn get_graphql_queries() -> Vec<String> {
    let mut requests = Vec::new();

    requests.push(
        r#"
        query allCustomers {
            customers {
              name
              id
              email
              address
              orders {
                id
                products{
                  id
                  name
                  price
                  description
                }
              }
            }
          }
        "#.into()
    );

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

    requests
}

pub fn get_rest_queries() -> Vec<(Method, String)> {
    let mut requests = Vec::new();
    requests.push((Method::GET, HOST_NAME.to_string() + "/customers"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/orders"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/products"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/products?min=1.0&max=50.0"));
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

pub fn build_rest_request(client: &Client, method: &Method, query: &str) -> Request {
    client.request(method.clone(), query).build().unwrap()
}
