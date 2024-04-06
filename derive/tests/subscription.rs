use dynamic_graphql::dynamic::{FieldValue, SubscriptionFieldFuture, TypeRef};
use dynamic_graphql::internal::{Register, Registry};
use dynamic_graphql::SimpleObject;
use dynamic_graphql::{value, App};
use futures_util::StreamExt;

use crate::schema_utils::normalize_schema;

mod schema_utils;
#[tokio::test]
async fn test_schema() {
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    struct Subscription;

    impl Register for Subscription {
        fn register(registry: Registry) -> Registry {
            let subscription = dynamic_graphql::dynamic::Subscription::new("Subscription");
            let field = dynamic_graphql::dynamic::SubscriptionField::new(
                "foo",
                TypeRef::named_nn(TypeRef::STRING),
                |_ctx| {
                    SubscriptionFieldFuture::new(async {
                        Ok(async_stream::try_stream! {
                            for i in 0..10 {
                                yield FieldValue::value(i.to_string());
                            }
                        })
                    })
                },
            );
            let subscription = subscription.field(field);
            let registry = registry.set_subscription("Subscription");
            registry.register_type(subscription)
        }
    }

    #[derive(App)]
    struct App(Query, Subscription);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(
        normalize_schema(&sdl),@r###"

    type Query {
      foo: String!
    }

    type Subscription {
      foo: String!
    }

    schema {
      query: Query
      subscription: Subscription
    }
    "###);

    let mut stream = schema.execute_stream("subscription { foo }");
    for i in 0..10 {
        let res = stream.next().await.unwrap().into_result().unwrap().data;
        assert_eq!(res, value!({ "foo": i.to_string() }));
    }
}
