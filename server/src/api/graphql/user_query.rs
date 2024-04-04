use std::sync::Arc;

use async_graphql::{Context, Object, Result, SimpleObject};
use mosscore::config::preference_file::BehaverPreferenceFile;

use crate::api::service::UserService;

#[derive(Default)]
pub struct UserQuery;

#[derive(Clone, SimpleObject, Debug)]
pub struct BehaverPreferenceFileGraphQL {
    pub visual: VisualBehaverPreferenceGraphQL,
    pub notification: NotificationBehaverPreferenceGraphQL,
}

#[derive(Clone, SimpleObject, Debug)]
pub struct VisualBehaverPreferenceGraphQL {
    theme: String,
}

#[derive(Clone, SimpleObject, Debug)]
pub struct NotificationBehaverPreferenceGraphQL {
    sound: bool,
}

impl From<BehaverPreferenceFile> for BehaverPreferenceFileGraphQL {
    fn from(file: BehaverPreferenceFile) -> Self {
        BehaverPreferenceFileGraphQL {
            visual: VisualBehaverPreferenceGraphQL {
                theme: file.visual.theme,
            },
            notification: NotificationBehaverPreferenceGraphQL {
                sound: file.notification.sound,
            },
        }
    }
}

#[Object]
impl UserQuery {
    async fn get_all_preference_category(
        &self,
        ctx: &Context<'_>,
    ) -> Result<BehaverPreferenceFileGraphQL> {
        let user_service = ctx.data::<Arc<UserService>>()?;
        let result: BehaverPreferenceFileGraphQL = (*user_service.user_settings).clone().into();

        Ok(result)
    }
}
