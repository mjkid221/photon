use crate::api::api::PhotonApi;
use crate::api::method::get_compressed_accounts_by_owner::PaginatedAccountList;
use crate::api::method::get_compressed_token_account_balance::TokenAccountBalance;
use crate::api::method::get_multiple_compressed_account_proofs::MerkleProofWithContext;
use crate::api::method::get_multiple_compressed_accounts::AccountList;
use crate::api::method::utils::Account;

use crate::api::method::utils::Context;
use crate::api::method::utils::Limit;
use crate::api::method::utils::TokenAcccount;
use crate::api::method::utils::TokenAccountList;
use crate::common::typedefs::hash::Hash;
use crate::common::typedefs::serializable_pubkey::SerializablePubkey;
use dirs;

use crate::common::relative_project_path;

use utoipa::openapi::path::OperationBuilder;

use utoipa::openapi::request_body::RequestBodyBuilder;

use utoipa::openapi::ContentBuilder;

use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::PathItem;
use utoipa::openapi::PathItemType;

use utoipa::openapi::RefOr;
use utoipa::openapi::Required;
use utoipa::openapi::ResponseBuilder;
use utoipa::openapi::ResponsesBuilder;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaType;

use utoipa::openapi::ServerBuilder;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(
    SerializablePubkey,
    Context,
    Hash,
    PaginatedAccountList,
    Account,
    MerkleProofWithContext,
    TokenAccountList,
    TokenAcccount,
    TokenAccountBalance,
    AccountList,
    Limit
)))]
struct ApiDoc;

fn add_string_property(
    builder: ObjectBuilder,
    name: &str,
    value: &str,
    description: &str,
) -> ObjectBuilder {
    let string_object = ObjectBuilder::new()
        .schema_type(SchemaType::String)
        .description(Some(description.to_string()))
        .enum_values(Some(vec![value.to_string()]))
        .build();

    let string_schema = RefOr::T(Schema::Object(string_object));
    builder.property(name, string_schema)
}

fn request_schema(name: &str, params: Option<RefOr<Schema>>) -> RefOr<Schema> {
    let mut builder = ObjectBuilder::new();

    builder = add_string_property(
        builder,
        "jsonrpc",
        "2.0",
        "The version of the JSON-RPC protocol.",
    );
    builder = add_string_property(builder, "id", "string", "An ID to identify the request.");
    builder = add_string_property(
        builder,
        "method",
        &name,
        "The name of the method to invoke.",
    );
    builder = builder
        .required("jsonrpc")
        .required("id")
        .required("method");

    if let Some(params) = params {
        builder = builder.property("params", params);
        builder = builder.required("params");
    }

    RefOr::T(Schema::Object(builder.build()))
}

pub fn update_docs(is_test: bool) {
    let method_api_specs = PhotonApi::method_api_specs();

    for spec in method_api_specs {
        let mut doc = ApiDoc::openapi();
        let content = ContentBuilder::new()
            .schema(request_schema(&spec.name, spec.request))
            .build();
        let json_content_type = "application/json".to_string();
        let request_body = RequestBodyBuilder::new()
            .content(json_content_type.clone(), content)
            .required(Some(Required::True))
            .build();
        let responses = ResponsesBuilder::new().response(
            "200",
            ResponseBuilder::new().content(
                json_content_type,
                ContentBuilder::new().schema(spec.response).build(),
            ),
        );
        let operation = OperationBuilder::new()
            .request_body(Some(request_body))
            .responses(responses)
            .build();
        let mut path_item = PathItem::new(PathItemType::Post, operation);

        path_item.summary = Some(spec.name.clone());
        doc.paths.paths.insert("/".to_string(), path_item);
        doc.servers = Some(vec![ServerBuilder::new()
            .url("http://127.0.0.1".to_string())
            .build()]);
        let yaml = doc.to_yaml().unwrap();

        let path = match is_test {
            true => {
                let tmp_directory = dirs::home_dir().unwrap().join(".tmp");

                // Create tmp directory if it does not exist
                if !tmp_directory.exists() {
                    std::fs::create_dir(&tmp_directory).unwrap();
                }

                relative_project_path(&format!(
                    "{}/{}.test.yaml",
                    tmp_directory.display(),
                    spec.name.clone()
                ))
            }
            false => {
                relative_project_path(&format!("src/openapi/specs/{}.yaml", spec.name.clone()))
            }
        };

        std::fs::write(path.clone(), yaml).unwrap();

        // Call the external swagger-cli validate command and fail if it fails
        let validate_result = std::process::Command::new("swagger-cli")
            .arg("validate")
            .arg(path.to_str().unwrap())
            .output()
            .unwrap();

        if !validate_result.status.success() {
            let stderr = String::from_utf8_lossy(&validate_result.stderr);
            panic!(
                "Failed to validate OpenAPI schema for {}. {}",
                spec.name, stderr
            );
        }
    }
}
