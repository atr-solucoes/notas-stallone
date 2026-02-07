use axum::{
    body::Body,
    http::{header, Response, StatusCode, Uri},
    response::IntoResponse,
    Router,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "temp_book_src/"] 
struct Asset;

#[tokio::main]
async fn main() {
    // 1. Porta Dinâmica
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!("🚀 Manual iniciado em http://{}", addr);

    let app = Router::new().fallback(static_handler);

    // 2. Abre o navegador usando a porta real que foi alocada
    if let Err(e) = webbrowser::open(&format!("http://{}", addr)) {
        eprintln!("Erro ao abrir navegador: {}", e);
    }

    // Inicia o servidor
    axum::serve(listener, app).await.unwrap();
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();

    // Retry Logic para encontrar o arquivo certo
    
    // Tentativa 1: O caminho exato ou raiz
    let attempt_path = if path.is_empty() || path.ends_with('/') {
        format!("{}index.html", path)
    } else {
        path.clone()
    };

    if let Some(response) = serve_file(&attempt_path) {
        return response;
    }

    // Tentativa 2: Se não achou 'pasta', tenta 'pasta/index.html'
    if !path.ends_with(".html") {
        let index_fallback = format!("{}/index.html", path);
        if let Some(response) = serve_file(&index_fallback) {
            return response;
        }
    }

    // Se falhar tudo: 404
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("404 - Página não encontrada no Manual"))
        .unwrap()
}

// Função auxiliar para evitar repetição de código
fn serve_file(path: &str) -> Option<Response<Body>> {
    match Asset::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Some(Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                // Cache Control: Ajuda a carregar SVGs instantaneamente ao navegar
                .header(header::CACHE_CONTROL, "public, max-age=3600") 
                .body(Body::from(content.data))
                .unwrap())
        }
        None => None,
    }
}
