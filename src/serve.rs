use actix_files as fs;
use actix_web::{App, HttpServer};
use std::path::Path;

pub async fn serve_directory<P: AsRef<Path>>(
    path: P,
    start_port: u16,
) -> Result<(), std::io::Error> {
    let mut port = start_port;

    loop {
        let address = path.as_ref().to_path_buf();
        match HttpServer::new(move || {
            App::new().service(
                fs::Files::new("/", &address)
                    .show_files_listing()
                    .index_file("index.html"),
            )
        })
        .bind(("127.0.0.1", port))
        {
            Ok(server) => {
                println!("Server running at http://localhost:{}", port);
                println!("Serving {}", path.as_ref().display());
                println!("Press Ctrl+C to stop");
                return server.run().await;
            }
            Err(_) => port += 1,
        }
    }
}
