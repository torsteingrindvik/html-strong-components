use axum::http::StatusCode;
use axum::routing::get_service;
use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*, template};
use html_strong_components::{TableTabs, Tabs};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler)).nest(
        "/static",
        get_service(ServeDir::new(".")).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }),
    );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_response(contents: Node) -> Html<String> {
    let response = contents
        .render_string()
        .expect("Should render successfully");

    Html(response)
}

fn make_table(name: &str) -> Node {
    let mut table = TableTabs::new(vec![
        "DESCRIPTION".into(),
        "Bing".into(),
        "Bang".into(),
        "Bong".into(),
    ]);

    for r in 0..5 {
        let mut row = vec![Pre
            .text("• Width fixed ✔\n• Height fixed ✔\n• Content scrollable ✔\n• Content any node ✔")
            .style("background: #fff;")
            .class("rounded")];

        for c in 0..3 {
            let tab_contents = (0..5)
                .into_iter()
                .map(|n| {
                    if n % 2 == 0 {
                        (
                            "Paragraph".to_string(),
                            P.text(format!("Paragraph #{n}")).into_node(),
                        )
                    } else {
                        (
                            "Image".to_string(),
                            Img::new("https://sbarthel.github.io/images/img/berg.JPG").into_node(),
                        )
                    }
                })
                .collect();
            let cell = Tabs::new(format!("{name}{r}{c}"), tab_contents);

            row.push(cell.into_node());
        }

        table.add_row(row);
    }

    table.into_node().class("hs-table")
}

async fn handler() -> Html<String> {
    let tables = Div
        .kid(make_table("TableUno"))
        .kid(make_table("TableDuo"))
        .kid(make_table("TableTrio"));

    get_response(
        template::HtmlDocumentBuilder::new()
            .with_body(tables)
            .with_head(template::head().kid(Link::stylesheet("text/css", "/static/style.css")))
            .build(),
    )
}
