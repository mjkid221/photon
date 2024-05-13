use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    super::error::PhotonApiError,
    utils::{
        search_for_signatures, Context, GetPaginatedSignaturesResponse, Limit, SignatureFilter,
        SignatureSearchType,
    },
};
use crate::common::typedefs::serializable_pubkey::SerializablePubkey;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct GetCompressionSignaturesForAddressRequest {
    pub address: SerializablePubkey,
    pub limit: Option<Limit>,
    pub cursor: Option<String>,
}

pub async fn get_compression_signatures_for_address(
    conn: &DatabaseConnection,
    request: GetCompressionSignaturesForAddressRequest,
) -> Result<GetPaginatedSignaturesResponse, PhotonApiError> {
    let context = Context::extract(conn).await?;

    let signatures = search_for_signatures(
        conn,
        SignatureSearchType::Standard,
        Some(SignatureFilter::Address(request.address)),
        request.cursor,
        request.limit,
    )
    .await?;

    Ok(GetPaginatedSignaturesResponse {
        value: signatures,
        context,
    })
}
