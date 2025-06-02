use std::fs;
use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::syntactic_analyzer::Parser;

use super::*;

pub struct RequestHandler {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, Vec<u8>>>>,
}

impl RequestHandler {
    pub fn new(client: Client) -> RequestHandler {
        RequestHandler {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for RequestHandler {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("HDDL Server".to_string()),
                        inter_file_dependencies: true,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                    },
                )),
                // Add other capabilities as needed
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "HDDL Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        ()
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        if self
            .documents
            .read()
            .await
            .get(&params.text_document.uri)
            .is_none()
        {
            self.documents.write().await.insert(
                params.text_document.uri,
                params.text_document.text.into_bytes(),
            );
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {}

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> tower_lsp::jsonrpc::Result<DocumentDiagnosticReportResult> {
        match self.documents.read().await.get(&params.text_document.uri) {
            Some(document) => {
                let file_path = params.text_document.uri.to_file_path().unwrap();
                let current_file_lexer = LexicalAnalyzer::new(document);
                let current_file_parser = Parser::new(current_file_lexer);
                match current_file_parser.classify() {
                    FileVariant::Domain => {
                        self.client
                            .log_message(
                                MessageType::INFO,
                                format!("{} is domain", file_path.display()),
                            )
                            .await;
                    }
                    FileVariant::Problem => {
                        let root_folder = file_path.parent().unwrap();
                        let mut files = tokio::fs::read_dir(root_folder).await.unwrap();
                        while let Ok(Some(entry)) = files.next_entry().await {
                            match entry.path().extension() {
                                Some(extension) if (extension == "hddl" || extension == "pddl") => {
                                    let content = tokio::fs::read(entry.path()).await.unwrap();
                                    let lexer = LexicalAnalyzer::new(&content);
                                    let parser = Parser::new(lexer);
                                    match parser.classify() {
                                        FileVariant::Domain => {
                                            self.client
                                                .log_message(
                                                    MessageType::INFO,
                                                    format!("{} is domain", entry.path().display()),
                                                )
                                                .await;
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    },
                    FileVariant::MaybeNotHDDL => {}
                }
            }
            None => {
                return Err(tower_lsp::jsonrpc::Error::invalid_params(format!(
                    "{} is not synced",
                    params.text_document.uri
                )));
            }
        }
        return Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    items: vec![Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 0,
                            },
                            end: Position {
                                line: 10,
                                character: 5,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(tower_lsp::lsp_types::NumberOrString::String(
                            "E001".to_string(),
                        )),
                        code_description: None,
                        source: Some("HDDL Analyzer".to_string()),
                        message: "This is a dummy error message".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                    }],
                    result_id: None,
                },
            }),
        ));
    }
}
