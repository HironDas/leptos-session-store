#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{ Router, routing::post};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes, file_and_error_handler};
    use leptos_session_store::{app::*};

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    // We don't use an address for the lambda function
    #[allow(unused_variables)]
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/{*path}", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options);

    // In development, we use the Hyper server
    #[cfg(debug_assertions)]
    {
        log::info!("listening on http://{}", &addr);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }

    // In release, we use the lambda_http crate
    #[cfg(not(debug_assertions))]
    {
        use lambda_runtime::{run, service_fn};
        use aws_lambda_events::encodings::http::Body;
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);

        run(app).await.unwrap();
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
