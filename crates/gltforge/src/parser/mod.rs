use std::panic::Location;
use std::path::Path;

use error_location::ErrorLocation;

use crate::error::parse::{ParseError, Result as ParseResult};
use crate::schema::{Accessor, Buffer, BufferView, Gltf};

/// Parse a glTF 2.0 JSON string into a [`Gltf`] document.
#[track_caller]
pub fn parse(json: &str) -> ParseResult<Gltf> {
    serde_json::from_str(json).map_err(|e| ParseError::Json {
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })
}

/// Load all external binary buffers referenced by a [`Gltf`] document.
///
/// `base_dir` must be the directory containing the `.gltf` file so that
/// relative URIs in [`Buffer::uri`] can be resolved.
pub fn load_buffers(gltf: &Gltf, base_dir: &Path) -> ParseResult<Vec<Vec<u8>>> {
    let Some(buffers) = gltf.buffers.as_deref() else {
        return Ok(Vec::new());
    };
    buffers.iter().map(|b| load_buffer(b, base_dir)).collect()
}

#[track_caller]
fn load_buffer(buf: &Buffer, base_dir: &Path) -> ParseResult<Vec<u8>> {
    let location = ErrorLocation::from(Location::caller());
    let uri = match &buf.uri {
        Some(u) => u,
        None => return Err(ParseError::GlbBufferNotSupported { location }),
    };
    if uri.starts_with("data:") {
        return Err(ParseError::DataUriNotSupported { location });
    }
    std::fs::read(base_dir.join(uri)).map_err(|e| ParseError::Io {
        source: e,
        location,
    })
}

/// Resolve an [`Accessor`] to its raw bytes.
///
/// Returns a slice into the appropriate region of the backing buffer. The
/// slice is tightly packed (stride is not yet supported).
#[track_caller]
pub fn resolve_accessor<'a>(
    accessor: &Accessor,
    buffer_views: &[BufferView],
    buffers: &'a [Vec<u8>],
) -> ParseResult<&'a [u8]> {
    let bv_index = accessor
        .buffer_view
        .ok_or_else(|| ParseError::AccessorNoBufferView {
            location: ErrorLocation::from(Location::caller()),
        })? as usize;

    let bv = buffer_views
        .get(bv_index)
        .ok_or_else(|| ParseError::BufferViewIndexOutOfRange {
            index: bv_index,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let buf_index = bv.buffer as usize;
    let buf = buffers
        .get(buf_index)
        .ok_or_else(|| ParseError::BufferIndexOutOfRange {
            index: buf_index,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let start = bv.byte_offset as usize + accessor.byte_offset.unwrap_or(0) as usize;
    let element_size =
        accessor.component_type.byte_size() * accessor.accessor_type.component_count();
    let end = start + accessor.count as usize * element_size;

    if end > buf.len() {
        return Err(ParseError::AccessorOutOfBounds {
            start,
            end,
            len: buf.len(),
            location: ErrorLocation::from(Location::caller()),
        });
    }

    Ok(&buf[start..end])
}
