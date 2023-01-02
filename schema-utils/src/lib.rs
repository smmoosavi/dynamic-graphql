use graphql_parser::schema::parse_schema;

pub fn normalize_schema(sdl: &str) -> String {
    let ast = parse_schema::<String>(sdl).unwrap().to_owned();
    format!("\n{}", ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_schema() {
        let sdl = "


            type Query {
                hello: String!
                nice: String!
                bye: String!
            }

            schema { query: Query }
        ";
        assert_eq!(
            normalize_schema(sdl),
            "
type Query {
  hello: String!
  nice: String!
  bye: String!
}

schema {
  query: Query
}
"
        );
    }
}
