use text::{TextUnit, TextRange};
use syntax_kinds::syntax_info;

use std::fmt;

mod file_builder;
pub use self::file_builder::{FileBuilder, Sink};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxKind(pub(crate) u32);

impl SyntaxKind {
    fn info(self) -> &'static SyntaxInfo {
        syntax_info(self)
    }
}

impl fmt::Debug for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.info().name;
        f.write_str(name)
    }
}


pub(crate) struct SyntaxInfo {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: TextUnit,
}

pub struct File {
	text: String,
	nodes: Vec<NodeData>,
	errors: Vec<SyntaxErrorData>,
}

impl File {
	pub fn root<'f>(&'f self) -> Node<'f> {
		assert!(!self.nodes.is_empty());
		Node { file: self, idx: NodeIdx(0) }
	}
}

#[derive(Clone, Copy)]
pub struct Node<'f> {
	file: &'f File,
	idx: NodeIdx,
}

impl<'f> Node<'f> {
	pub fn kind(&self) -> SyntaxKind {
		self.data().kind
	}

	pub fn range(&self) -> TextRange {
		self.data().range
	}

	pub fn text(&self) -> &'f str {
		&self.file.text.as_str()[self.range()]
	}

	pub fn parent(&self) -> Option<Node<'f>> {
		self.as_node(self.data().parent)
	}

	pub fn children(&self) -> Children<'f> {
		Children { next: self.as_node(self.data().first_child) }
	}

	pub fn SyntaxErrors(&self) -> SyntaxErrors<'f> {
		let pos = self.file.errors.iter().position(|e| e.node == self.idx);
		let next = pos
			.map(|i| ErrorIdx(i as u32))
			.map(|idx| SyntaxError { file: self.file, idx });
		SyntaxErrors { next }
	}

	fn data(&self) -> &'f NodeData {
		&self.file.nodes[self.idx]
	}

	fn as_node(&self, idx: Option<NodeIdx>) -> Option<Node<'f>> {
		idx.map(|idx| Node { file: self.file, idx })
	}
}

impl<'f> fmt::Debug for Node<'f> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{:?}@{:?}", self.kind(), self.range())
	}
}

#[derive(Clone, Copy)]
pub struct SyntaxError<'f> {
	file: &'f File,
	idx: ErrorIdx,
}

impl<'f> SyntaxError<'f> {
	pub fn message(&self) -> &'f str {
		self.data().message.as_str()
	}

	fn data(&self) -> &'f SyntaxErrorData {
		&self.file.errors[self.idx]
	}

	fn next(&self) -> Option<SyntaxError<'f>> {
		if self.file.errors.len() == self.idx.0 as usize {
			return None;
		}
		let result = SyntaxError {
			file: self.file,
			idx: ErrorIdx(self.idx.0 + 1)
		};
		if result.data().node != self.data().node {
			return None;
		}
		Some(result)
	}
}

pub struct Children<'f> {
	next: Option<Node<'f>>,
}

impl<'f> Iterator for Children<'f> {
	type Item = Node<'f>;

	fn next(&mut self) -> Option<Node<'f>> {
		let next = self.next;
		self.next = next.and_then(|node| node.as_node(node.data().next_sibling));
		next
	}
}

pub struct SyntaxErrors<'f> {
	next: Option<SyntaxError<'f>>,
}

impl<'f> Iterator for SyntaxErrors<'f> {
	type Item = SyntaxError<'f>;

	fn next(&mut self) -> Option<SyntaxError<'f>> {
		let next = self.next;
		self.next = next.as_ref().and_then(SyntaxError::next);
		next
	}
}


#[derive(Clone, Copy, PartialEq, Eq)]
struct NodeIdx(u32);

struct NodeData {
	kind: SyntaxKind,
	range: TextRange,
	parent: Option<NodeIdx>,
	first_child: Option<NodeIdx>,
	next_sibling: Option<NodeIdx>,
}

impl ::std::ops::Index<NodeIdx> for Vec<NodeData> {
	type Output = NodeData;

	fn index(&self, NodeIdx(idx): NodeIdx) -> &NodeData {
		&self[idx as usize]
	}
}

impl ::std::ops::IndexMut<NodeIdx> for Vec<NodeData> {
	fn index_mut(&mut self, NodeIdx(idx): NodeIdx) -> &mut NodeData {
		&mut self[idx as usize]
	}
}

#[derive(Clone, Copy)]
struct ErrorIdx(u32);

struct SyntaxErrorData {
	node: NodeIdx,
	message: String,
}

impl ::std::ops::Index<ErrorIdx> for Vec<SyntaxErrorData> {
	type Output = SyntaxErrorData;

	fn index(&self, ErrorIdx(idx): ErrorIdx) -> &SyntaxErrorData {
		&self[idx as usize]
	}
}