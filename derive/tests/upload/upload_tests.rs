use std::io::Read;
use std::io::Write;

use dynamic_graphql::App;
use dynamic_graphql::Context;
use dynamic_graphql::InputObject;
use dynamic_graphql::Mutation;
use dynamic_graphql::MutationFields;
use dynamic_graphql::MutationRoot;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::Upload;
use dynamic_graphql::UploadValue;
use dynamic_graphql::Variables;

use crate::schema_utils::normalize_schema;

fn create_upload_value(content: String) -> (tempfile::NamedTempFile, UploadValue) {
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    temp_file.write_all(content.as_bytes()).unwrap();
    let path = temp_file.path();
    let file = std::fs::File::open(path).unwrap();

    let upload_value = UploadValue {
        filename: "test".to_string(),
        content_type: Some("text/plain".to_string()),
        content: file,
    };
    (temp_file, upload_value)
}

#[tokio::test]
async fn test_arg() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct UploadMutation(MutationRoot);

    #[MutationFields]
    impl UploadMutation {
        fn test(ctx: &Context, file: Upload) -> String {
            let mut upload_value = file.value(ctx).expect("upload value");
            let mut content = String::new();
            upload_value
                .content
                .read_to_string(&mut content)
                .expect("read content");
            let filename = upload_value.filename;
            let content_type = upload_value
                .content_type
                .unwrap_or_else(|| "unknown".to_string());
            format!("filename:{filename}\ncontent_type:{content_type}\ncontent:{content}")
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, UploadMutation);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type MutationRoot {
      test(file: Upload!): String!
    }

    type Query {
      foo: String!
    }

    scalar Upload

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @specifiedBy(url: String!) on SCALAR

    schema {
      query: Query
      mutation: MutationRoot
    }
    ");

    let query = r##"
        mutation($file: Upload!) {
            test(file: $file)
        }
        "##;

    let (_temp_file, upload_value) = create_upload_value("the content".to_string());

    let variables = serde_json::json!({ "file": null });
    let mut req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));
    req.set_upload("variables.file", upload_value);
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    let content = "filename:test\ncontent_type:text/plain\ncontent:the content".to_string();
    assert_eq!(res.errors.len(), 0);
    assert_eq!(data, serde_json::json!({ "test": content }));
}

#[tokio::test]
async fn test_input_object() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct UploadMutation(MutationRoot);

    #[derive(InputObject)]
    struct UploadInput {
        file: Upload,
    }

    #[MutationFields]
    impl UploadMutation {
        fn test(ctx: &Context, input: UploadInput) -> String {
            let mut upload_value = input.file.value(ctx).expect("upload value");
            let mut content = String::new();
            upload_value
                .content
                .read_to_string(&mut content)
                .expect("read content");
            let filename = upload_value.filename;
            let content_type = upload_value
                .content_type
                .unwrap_or_else(|| "unknown".to_string());
            format!("filename:{filename}\ncontent_type:{content_type}\ncontent:{content}")
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, UploadMutation);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type MutationRoot {
      test(input: UploadInput!): String!
    }

    type Query {
      foo: String!
    }

    scalar Upload

    input UploadInput {
      file: Upload!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @specifiedBy(url: String!) on SCALAR

    schema {
      query: Query
      mutation: MutationRoot
    }
    ");

    let query = r##"
        mutation($input: UploadInput!) {
            test(input: $input)
        }
        "##;

    let (_temp_file, upload_value) = create_upload_value("the content".to_string());

    let variables = serde_json::json!({ "input": { "file": null} });
    let mut req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));
    req.set_upload("variables.input.file", upload_value);
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    let content = "filename:test\ncontent_type:text/plain\ncontent:the content".to_string();
    assert_eq!(res.errors.len(), 0);
    assert_eq!(data, serde_json::json!({ "test": content }));
}

#[tokio::test]
async fn test_arg_optional() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct UploadMutation(MutationRoot);

    #[MutationFields]
    impl UploadMutation {
        fn test(ctx: &Context, file: Option<Upload>) -> String {
            let Some(file) = file else {
                return "no file".to_string();
            };
            let mut upload_value = file.value(ctx).expect("upload value");
            let mut content = String::new();
            upload_value
                .content
                .read_to_string(&mut content)
                .expect("read content");
            let filename = upload_value.filename;
            let content_type = upload_value
                .content_type
                .unwrap_or_else(|| "unknown".to_string());
            format!("filename:{filename}\ncontent_type:{content_type}\ncontent:{content}")
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, UploadMutation);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type MutationRoot {
      test(file: Upload): String!
    }

    type Query {
      foo: String!
    }

    scalar Upload

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @specifiedBy(url: String!) on SCALAR

    schema {
      query: Query
      mutation: MutationRoot
    }
    ");

    let query = r##"
        mutation($file: Upload!) {
            test(file: $file)
        }
        "##;

    let variables = serde_json::json!({ "file": null });
    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    let content = "no file".to_string();
    assert_eq!(res.errors.len(), 0);
    assert_eq!(data, serde_json::json!({ "test": content }));

    let (_temp_file, upload_value) = create_upload_value("the content".to_string());

    let variables = serde_json::json!({ "file": null });
    let mut req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));
    req.set_upload("variables.file", upload_value);
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    let content = "filename:test\ncontent_type:text/plain\ncontent:the content".to_string();
    assert_eq!(res.errors.len(), 0);
    assert_eq!(data, serde_json::json!({ "test": content }));
}

#[tokio::test]
async fn test_input_object_optional() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct UploadMutation(MutationRoot);

    #[derive(InputObject)]
    struct UploadInput {
        file: Option<Upload>,
    }

    #[MutationFields]
    impl UploadMutation {
        fn test(ctx: &Context, input: UploadInput) -> String {
            let Some(file) = input.file else {
                return "no file".to_string();
            };
            let mut upload_value = file.value(ctx).expect("upload value");
            let mut content = String::new();
            upload_value
                .content
                .read_to_string(&mut content)
                .expect("read content");
            let filename = upload_value.filename;
            let content_type = upload_value
                .content_type
                .unwrap_or_else(|| "unknown".to_string());
            format!("filename:{filename}\ncontent_type:{content_type}\ncontent:{content}")
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, UploadMutation);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type MutationRoot {
      test(input: UploadInput!): String!
    }

    type Query {
      foo: String!
    }

    scalar Upload

    input UploadInput {
      file: Upload
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @specifiedBy(url: String!) on SCALAR

    schema {
      query: Query
      mutation: MutationRoot
    }
    ");

    let query = r##"
        mutation($input: UploadInput!) {
            test(input: $input)
        }
        "##;

    let variables = serde_json::json!({ "input": { "file": null} });
    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    let content = "no file".to_string();
    assert_eq!(res.errors.len(), 0);
    assert_eq!(data, serde_json::json!({ "test": content }));

    let (_temp_file, upload_value) = create_upload_value("the content".to_string());

    let variables = serde_json::json!({ "input": { "file": null} });
    let mut req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));
    req.set_upload("variables.input.file", upload_value);
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    let content = "filename:test\ncontent_type:text/plain\ncontent:the content".to_string();
    assert_eq!(res.errors.len(), 0);
    assert_eq!(data, serde_json::json!({ "test": content }));
}
