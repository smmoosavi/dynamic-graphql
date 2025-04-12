pub fn normalize_schema(sdl: &str) -> String {
    format!(
        "{}",
        graphql_parser::schema::parse_schema::<String>(sdl)
            .unwrap()
            .to_owned()
    )
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
        insta::assert_snapshot!(normalize_schema(sdl), @r"
        type Query {
          hello: String!
          nice: String!
          bye: String!
        }

        schema {
          query: Query
        }
        ");
    }
}
