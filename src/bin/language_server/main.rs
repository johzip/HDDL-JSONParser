use tower_lsp::{LspService, Server};

use hddl_analyzer::RequestHandler;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| RequestHandler::new(client));

    Server::new(stdin, stdout, socket).serve(service).await;
}
