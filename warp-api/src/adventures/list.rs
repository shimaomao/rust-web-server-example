use domain::{AdventuresManager, AdventuresQuery};
use serde::Deserialize;
use tracing::debug;
use types::my_item_type_format::to_item_type_name;
use validator::{Validate, ValidationError};

use crate::{adventures::response::AdventuresResponse, response::ErrorResponse, AppState};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct AdventuresQueryReq {
    #[validate(custom(function = "validate_item_id", message = "item_id is not correct"))]
    pub item_id: u8,
    #[validate(range(min = 1, max = 20, message = "limit 1 - 20"))]
    pub limit: Option<u32>,
    #[validate(range(min = 0, message = "offset start at 0"))]
    pub offset: Option<u32>,
    #[validate(length(min = 2, message = "province_key at lease 2 chars"))]
    pub province_key: Option<String>,
}

impl From<AdventuresQueryReq> for AdventuresQuery {
    fn from(ad: AdventuresQueryReq) -> Self {
        Self {
            item_id: ad.item_id,
            limit: ad.limit,
            offset: ad.offset,
            province_key: ad.province_key,
        }
    }
}

fn validate_item_id(item_id: u8) -> Result<(), ValidationError> {
    if to_item_type_name(item_id.into()) == "" {
        return Err(ValidationError::new("item_id"));
    }

    Ok(())
}

#[tracing::instrument(skip(state))]
pub async fn list_adventures(
    query: AdventuresQueryReq,
    state: AppState,
) -> Result<impl warp::Reply, ErrorResponse> {
    debug!("query: {:?}, state: {:?}", query, state);
    let manager = &state.adventures_manager;
    let adventures = manager.find_adventures(query.into()).await?;
    let response = AdventuresResponse::from(adventures);
    debug!("response: {:?}", &response);
    Ok(response)
}
