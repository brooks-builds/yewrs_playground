use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/get_games.graphql",
    response_derives = "Debug"
)]
pub struct GetGames;
