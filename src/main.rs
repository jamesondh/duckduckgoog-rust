use warp::{http::StatusCode, http::Uri, Filter, Reply};

#[tokio::main]
async fn main() {
    let search = warp::path("search")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(
            |query: std::collections::HashMap<String, String>| -> Box<dyn Reply> {
                if let Some(q) = query.get("q") {
                    if q.starts_with('!') {
                        // Redirect to DuckDuckGo with the original query
                        Box::new(warp::redirect::temporary(
                            format!("https://duckduckgo.com/?q={}", urlencoding::encode(q))
                                .parse::<Uri>()
                                .unwrap(),
                        ))
                    } else {
                        // Redirect to Google by default
                        Box::new(warp::redirect::temporary(
                            format!("https://www.google.com/search?q={}", urlencoding::encode(q))
                                .parse::<Uri>()
                                .unwrap(),
                        ))
                    }
                } else {
                    Box::new(warp::reply::with_status(
                        "Query parameter 'q' missing",
                        StatusCode::BAD_REQUEST,
                    ))
                }
            },
        );

    println!("Started server on port 3030.");
    warp::serve(search).run(([127, 0, 0, 1], 3030)).await;
}
