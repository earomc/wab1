use reqwest::{
    blocking::{Client, Request},
    Method,
};
use serde_json::json;

use crate::{GRAPHQL_ENDPOINT, HOST_NAME};

pub fn get_graphql_queries() -> Vec<String> {
    let mut requests = Vec::new();

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
    "#
        .to_string(),
    );

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
        "#
        .to_string(),
    );

    requests.push(
        r#"
    query allOrders {
        orders {
          id
          orderedOn
          status
          products {
            id
            name
            price
            description
          }
        }
      }
    "#
        .to_string(),
    );

    requests.push(
        r#"
    query customerById {
        customer(id: "customer-6") {
          id
          name
          email
          address
          orders {
            id
            orderedOn
            status
            products {
              id
              name
              price
              description
            }
          }
        }
      }
    "#
        .to_string(),
    );

    requests.push(
        r#"
        query productById {
            product(id: "product-1") {
              id
              name
              price
              description
            }
          }
        "#
        .to_string(),
    );

    requests.push(
        r#"
        query orderById {
            order(id: "order-1") {
              id
              orderedOn
              status
              products {
                id
                name
                price
                description
              }
            }
          }
        "#
        .to_string(),
    );

    requests.push(
        r#"
        query {
            products(priceFilter: { min: 10, max: 50 }) {
                id
                name
                price
            }
        }
        "#
        .to_string(),
    );

    requests
}

pub fn get_rest_queries() -> Vec<(Method, String)> {
    let mut requests = Vec::new();
    requests.push((Method::GET, HOST_NAME.to_string() + "/products"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/customers"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/orders"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/customer?id=customer-6"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/product?id=product-1"));
    requests.push((Method::GET, HOST_NAME.to_string() + "/order?id=order-1"));
    requests.push((
        Method::GET,
        HOST_NAME.to_string() + "/products?min=1.0&max=50.0",
    ));
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
