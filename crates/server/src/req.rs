use languageserver_types::{TextDocumentIdentifier, Range};

pub use languageserver_types::{
    request::*, notification::*,
    InitializeResult,
};

pub enum SyntaxTree {}

impl Request for SyntaxTree {
    type Params = SyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "m/syntaxTree";
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreeParams {
    pub text_document: TextDocumentIdentifier
}

pub enum ExtendSelection {}

impl Request for ExtendSelection {
    type Params = ExtendSelectionParams;
    type Result = ExtendSelectionResult;
    const METHOD: &'static str = "m/extendSelection";
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtendSelectionParams {
    pub text_document: TextDocumentIdentifier,
    pub selections: Vec<Range>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtendSelectionResult {
    pub selections: Vec<Range>,
}