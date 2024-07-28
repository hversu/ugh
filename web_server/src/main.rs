extern crate dotenv;

use dotenv::dotenv;
use universal_graph_transformer::{graph_transformer, types::Graph};

mod template;

use uuid::Uuid;
use axum::{
    body::Bytes,
    extract::{Multipart},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    BoxError, Router,
};

use tower::ServiceExt;
use tower_http::{
    services::{ServeDir}
};
use futures::{Stream, TryStreamExt};
use std::io;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tokio::net::TcpListener;
use template::{HtmlTemplate, IndexTemplate};

const UPLOADS_DIRECTORY: &str = "uploads";
const OUTPUT_DIRECTORY: &str = "outputs";


#[tokio::main]
async fn main() -> io::Result<()>{
    dotenv().ok();
    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(accept_form))
        .nest_service(format!("/{}", OUTPUT_DIRECTORY.to_owned()).as_str(), ServeDir::new
            (OUTPUT_DIRECTORY.to_owned
        ()));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}

// Handler that accepts a multipart form upload and streams each field to a file.
async fn accept_form(mut multipart: Multipart) -> Result<String, (StatusCode, String)> {
    let mut file_path = UPLOADS_DIRECTORY.to_owned();
    let my_uuid = Uuid::new_v4();
    let output_path = format!("{}/{}.json", OUTPUT_DIRECTORY, my_uuid);
    while let Ok(Some(field)) = multipart.next_field().await {
        // Generate a random UUID
        let file_name = if let Some(file_name) = field.file_name() {
            format!("{}-{}", my_uuid, file_name.to_owned())
        } else {
            my_uuid.to_string()
        };
        file_path = format!("{}/{}", file_path, file_name);
        stream_to_file(&file_name, field).await?;
        match graph_transformer(&file_path, "auto", &output_path).await {
            Ok(_) => println!("Graph transformation successful"),
            Err(e) => eprintln!("An error occurred: {}", e),
        };
    }
    Ok(output_path)
}


// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}