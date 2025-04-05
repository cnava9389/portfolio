use crate::app::*;
use actix_files::Files;
use actix_web::{App as ActixApp, *};
use leptos::{config::get_configuration, prelude::*};
use leptos_actix::{LeptosRoutes, generate_route_list};
use leptos_meta::{MetaTags, Meta, Link, Script};

#[get("favicon.svg")]
async fn favicon(leptos_options: web::Data<LeptosOptions>) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.svg"
    ))?)
}

pub async fn run() -> std::io::Result<()> {
    _ = dotenvy::dotenv();
    env_logger::init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    log::info!("Serving at {}, {}", addr, conf.leptos_options.site_root);

    let req = web::Data::new(reqwest::Client::new());

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        ActixApp::new() 
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            //.service(Files::new("/", site_root.as_ref()))
            .service(Files::new("/assets", site_root.as_ref()))
            .service(favicon)
            .leptos_routes(routes, {
                let options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <Meta charset="utf-8" /> 
	                            <Meta name="viewport" content="width=device-wdith, inital-scale=1.0, maximum-scale=1" />
                                <Link rel="icon" href="/favicon.svg" type_="image/svg+xml" />
                                <AutoReload options=options.clone() />
                                <HydrationScripts options=options.clone() /> //islands=true />
                                <MetaTags />
                                <Script type_="module" src="/assets/main.js"/> 
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(req.clone())
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}
