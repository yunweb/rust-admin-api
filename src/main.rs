#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate serde_derive;

mod cli_args;
mod database;
mod errors;
mod graphql;
mod schema;
mod modules;
mod route;
mod jwt;

use actix_web::{ App, web, guard, HttpServer };
use async_graphql::{EmptyMutation, EmptySubscription, Schema};



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let port = opt.port;
    eprintln!("Listening on 0.0.0.0:{}", port);

    // Create Juniper schema
    // let schema = std::sync::Arc::new(crate::graphql::schemas::root::create_schema());
    let schema = Schema::new(crate::graphql::index::QueryRoot, EmptyMutation, EmptySubscription);
    HttpServer::new(move || {
        App::new()
            .data(database::pool::establish_connection(opt.clone()))
            .data(schema.clone())
            .data(opt.clone())
            .configure(route::index)
            // .configure(graphql::route)
            .service(web::resource("/graphql").guard(guard::Post()).to(crate::graphql::index::index))
            .service(web::resource("/graphiql").guard(guard::Get()).to(graphql::gql_playgound))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[cfg(test)]
mod tests;
