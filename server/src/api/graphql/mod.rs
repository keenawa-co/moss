mod inspector_query;

use async_graphql::MergedObject;

use self::inspector_query::InspectorQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(InspectorQuery);
