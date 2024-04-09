use reqwest::{
    blocking::{Client, Request},
    Method,
};
use serde_json::json;

use crate::{GRAPHQL_ENDPOINT, HOST_NAME};

pub fn get_graphql_queries() -> Vec<GraphQlQuery> {
    let mut gql_queries = Vec::new();

    gql_queries.push(GraphQlQuery {
        name: "all_products".to_string(),
        raw_query: r#"
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
    });

    gql_queries.push(GraphQlQuery {
        name: "all_customers".to_string(),
        raw_query: r#"
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
    });

    gql_queries.push(GraphQlQuery {
        name: "all_orders".to_string(),
        raw_query: r#"
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
    });

    gql_queries.push(GraphQlQuery {
        name: "customer_by_id".to_string(),
        raw_query: r#"
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
    });

    gql_queries.push(GraphQlQuery {
        name: "product_by_id".to_string(),
        raw_query: r#"
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
    });

    gql_queries.push(GraphQlQuery {
        name: "order_by_id".to_string(),
        raw_query: r#"
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
    });

    gql_queries.push(GraphQlQuery {
        name: "products_with_filter".to_string(),
        raw_query: r#"
        query productsWithFilter ($priceFilter: PriceFilter) {
            products(priceFilter: $priceFilter) {
                id
                name
                price
                description
            }
          }
        "#
        .to_string(),
    });
    gql_queries
}

pub fn get_rest_queries() -> Vec<RestQuery> {
    let mut requests = Vec::new();
    requests.push(RestQuery::new_get(
        HOST_NAME.to_string() + "/products",
        "all_products",
    ));
    requests.push(RestQuery::new_get(
        HOST_NAME.to_string() + "/customers",
        "all_customers",
    ));
    requests.push(RestQuery::new_get(
        HOST_NAME.to_string() + "/orders",
        "all_orders",
    ));
    requests.push(RestQuery::new_get(
        HOST_NAME.to_string() + "/customer?id=customer-6",
        "customer_by_id",
    ));
    requests.push(RestQuery::new_get(
        HOST_NAME.to_string() + "/product?id=product-1",
        "product_by_id",
    ));
    requests.push(RestQuery::new_get(
        HOST_NAME.to_string() + "/order?id=order-1",
        "order_by_id",
    ));
    requests.push(RestQuery::new_get(
        HOST_NAME.to_string() + "/products?min=1.0&max=50.0",
        "products_with_filter",
    ));
    requests
}

pub struct GraphQlQuery {
    pub raw_query: String,
    pub name: String,
}

impl GraphQlQuery {
    pub fn to_request(&self, client: &Client) -> Request {
        let body = json!({"query": self.raw_query});
    
        client
            .post(GRAPHQL_ENDPOINT)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .build()
            .unwrap()
    }
}

pub struct RestQuery {
    method: Method,
    pub endpoint: String,
    pub name: String,
}

impl RestQuery {
    fn new(method: Method, endpoint: String, name: &str) -> RestQuery {
        RestQuery {
            method,
            endpoint,
            name: name.to_string(),
        }
    }
    pub fn new_get(endpoint: String, name: &str) -> RestQuery {
        Self::new(Method::GET, endpoint, name)
    }
    pub fn new_put(endpoint: String, name: &str) -> RestQuery {
        Self::new(Method::PUT, endpoint, name)
    }
    pub fn new_delete(endpoint: String, name: &str) -> RestQuery {
        Self::new(Method::DELETE, endpoint, name)
    }
    pub fn new_post(endpoint: String, name: &str) -> RestQuery {
        Self::new(Method::POST, endpoint, name)
    }
    pub fn to_request(&self, client: &Client) -> Request {
        client.request(self.method().clone(), &self.endpoint).build().unwrap()
    }
}

impl MeasuredQuery for RestQuery {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn method(&self) -> &Method {
        &self.method
    }
}

impl MeasuredQuery for GraphQlQuery {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn method(&self) -> &Method {
        &Method::PUT
    }
}

trait MeasuredQuery {
    fn get_name(&self) -> &str;
    fn method(&self) -> &Method;
}
