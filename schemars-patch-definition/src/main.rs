use kube::CustomResource;
use kube::CustomResourceExt;
use lazy_static::lazy_static;
use regex::Regex;
use schemars::{schema_for, JsonSchema};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use validator::Validate;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Validate)]
#[kube(group = "suin.jp", version = "v1", kind = "Something", namespaced)]
#[kube(status = "SomethingStatus")]
struct SomethingSpec {
    #[schemars(schema_with = "my_validation")]
    start: i64,
    #[schemars(schema_with = "my_validation_by_macro")]
    end: i64,
    #[validate(length(min = 3), regex(path = "RE_TWO_CHARS"))]
    name: String,
}

lazy_static! {
    static ref RE_TWO_CHARS: Regex = Regex::new(r"[a-z]+$").unwrap();
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
struct SomethingStatus {}

// 通常版
fn my_validation(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = gen.subschema_for::<i64>().into_object().clone();
    schema.extensions = [(
        "x-kubernetes-validations".to_string(),
        json!([{"rule": "self.start < self.end", "message": "start must be less than end"}]),
    )]
    .into();
    schema.into()
}

// マクロ版
macro_rules! validator {
    ($name:ident, $rule:expr, $message:expr) => {
        fn $name(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            let mut schema = gen.subschema_for::<i64>().into_object().clone();
            schema.extensions = [(
                "x-kubernetes-validations".to_string(),
                json!([{"rule": $rule, "message": $message}]),
            )]
            .into_iter()
            .collect();
            schema.into()
        }
    };
}

validator!(
    my_validation_by_macro,
    "self.start < self.end",
    "start must be less than end"
);

fn main() {
    // let schema = schema_for!(SomethingSpec);
    // println!("{}", serde_yaml::to_string(&schema).unwrap());
    println!("{}", serde_yaml::to_string(&Something::crd()).unwrap());
}
