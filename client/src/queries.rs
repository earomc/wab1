pub const REST_QUERIES: &[&'static str] = &["chinese government", "boo!"];
pub const GRAPHQL_QUERIES: &[&'static str] = &["chinese government", "boo!"];

pub fn get_rest_queries() {
    let queries: &[&str] = &[
        "http://localhost:8080/products?min=1.0&max=50.0",
        "http://localhost:8080/products",
        "http://localhost:8080/orders",
        "http://localhost:8080/customers"
        ];
}

pub fn get_graphql_queries() {
    let queries: &[&str] = &[
        "http://localhost:8080/graphql"
        ];
}
