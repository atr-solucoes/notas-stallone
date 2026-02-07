fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        // O CI/CD vai passar o caminho do ícone aqui
        if let Ok(icon_path) = std::env::var("BOOK_ICON") {
            res.set_icon(&icon_path);
        }
        res.compile().unwrap();
    }
}
