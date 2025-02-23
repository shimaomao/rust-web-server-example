use axum::{extract::Extension, Json};
use domain::UsersManager;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    app_request::{AuthUser, ValidatedPath},
    app_response::AppError,
    AppState,
};

#[derive(Default, Deserialize, Debug, Clone, Validate)]
pub struct FavoriteForm {
    #[validate(range(min = 1, message = "id not correct"))]
    id: i64,
}

#[derive(Serialize)]
pub struct FavoriteResponse {
    favorited: bool,
}

enum Action {
    Favorite,
    Unfavorite,
}

#[tracing::instrument(skip(auth_user, state))]
pub async fn favorite(
    ValidatedPath(form): ValidatedPath<FavoriteForm>,
    AuthUser(auth_user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<FavoriteResponse>, AppError> {
    process(form, AuthUser(auth_user), state, Action::Favorite).await
}

#[tracing::instrument(skip(auth_user, state))]
pub async fn unfavorite(
    ValidatedPath(form): ValidatedPath<FavoriteForm>,
    AuthUser(auth_user): AuthUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<FavoriteResponse>, AppError> {
    process(form, AuthUser(auth_user), state, Action::Unfavorite).await
}

async fn process(
    form: FavoriteForm,
    AuthUser(auth_user): AuthUser,
    state: AppState,
    action: Action,
) -> Result<Json<FavoriteResponse>, AppError> {
    let manager = &state.users_manager;
    let user = manager.get_user_by_username(auth_user.get_name()).await?;

    match action {
        Action::Favorite => {
            user.favorite(form.id, &state.favorites_manager).await?;

            Ok(FavoriteResponse { favorited: true }.into())
        }
        Action::Unfavorite => {
            user.unfavorite(form.id, &state.favorites_manager).await?;

            Ok(FavoriteResponse { favorited: false }.into())
        }
    }
}
