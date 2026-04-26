//! Minimal `typst::World` impl serving an in-memory main.typ + bundled
//! `@preview/cetz` and `@preview/oxifmt` packages from `crates/dsl/assets/`.

use std::sync::OnceLock;

use include_dir::{Dir, include_dir};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

static CETZ: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/cetz-0.5.0");
static OXIFMT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/oxifmt-1.0.0");

fn fonts() -> &'static (LazyHash<FontBook>, Vec<Font>) {
    static CELL: OnceLock<(LazyHash<FontBook>, Vec<Font>)> = OnceLock::new();
    CELL.get_or_init(|| {
        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data)))
            .collect();
        let book = FontBook::from_fonts(&fonts);
        (LazyHash::new(book), fonts)
    })
}

fn library() -> &'static LazyHash<Library> {
    static CELL: OnceLock<LazyHash<Library>> = OnceLock::new();
    CELL.get_or_init(|| LazyHash::new(<Library as LibraryExt>::default()))
}

pub struct InMemoryWorld {
    main_id: FileId,
    main_src: Source,
}

impl InMemoryWorld {
    pub fn new(src: String) -> Self {
        let main_id = FileId::new(None, VirtualPath::new("/main.typ"));
        let main_src = Source::new(main_id, src);
        Self { main_id, main_src }
    }

    fn lookup_package(&self, spec: &PackageSpec, vpath: &VirtualPath) -> Option<&[u8]> {
        let dir = match (spec.namespace.as_str(), spec.name.as_str()) {
            ("preview", "cetz") if spec.version.to_string() == "0.5.0" => &CETZ,
            ("preview", "oxifmt") if spec.version.to_string() == "1.0.0" => &OXIFMT,
            _ => return None,
        };
        // VirtualPath like "/lib.typ" — strip leading slash for include_dir.
        let rooted = vpath.as_rooted_path();
        let rel = rooted.strip_prefix("/").ok()?;
        dir.get_file(rel).map(|f| f.contents())
    }
}

impl World for InMemoryWorld {
    fn library(&self) -> &LazyHash<Library> {
        library()
    }
    fn book(&self) -> &LazyHash<FontBook> {
        &fonts().0
    }
    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            return Ok(self.main_src.clone());
        }
        if let Some(spec) = id.package() {
            if let Some(bytes) = self.lookup_package(spec, id.vpath()) {
                let text = std::str::from_utf8(bytes)
                    .map_err(|_| FileError::InvalidUtf8)?
                    .to_string();
                return Ok(Source::new(id, text));
            }
        }
        Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(spec) = id.package() {
            if let Some(bytes) = self.lookup_package(spec, id.vpath()) {
                return Ok(Bytes::new(bytes.to_vec()));
            }
        }
        Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
    }

    fn font(&self, idx: usize) -> Option<Font> {
        fonts().1.get(idx).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}
