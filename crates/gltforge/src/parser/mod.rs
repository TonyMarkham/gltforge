use std::panic::Location;
use std::path::Path;

use error_location::ErrorLocation;

use crate::error::parse::{ParseError, Result as ParseResult};
use crate::schema::{Accessor, Buffer, BufferView, Gltf};

const GLB_MAGIC: &[u8; 4] = b"glTF";
const GLB_VERSION_2: u32 = 2;
const CHUNK_TYPE_JSON: u32 = 0x4E4F534A; // b"JSON"
const CHUNK_TYPE_BIN: u32 = 0x004E4942; // b"BIN\0"

/// Parse a glTF 2.0 JSON string into a [`Gltf`] document.
#[track_caller]
pub fn parse(json: &str) -> ParseResult<Gltf> {
    serde_json::from_str(json).map_err(|e| ParseError::Json {
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })
}

/// Parse a GLB (binary glTF 2.0) container.
///
/// Returns the parsed [`Gltf`] document and the raw binary buffer data.
/// `buffers[0]` contains the GLB BIN chunk data, if one is present.
///
/// For `.gltf` files with external `.bin` files, use [`parse`] + [`load_buffers`] instead.
#[track_caller]
pub fn parse_glb(data: &[u8]) -> ParseResult<(Gltf, Vec<Vec<u8>>)> {
    let location = ErrorLocation::from(Location::caller());

    if data.len() < 12 {
        return Err(ParseError::GlbTooShort { location });
    }

    if &data[0..4] != GLB_MAGIC {
        return Err(ParseError::GlbBadMagic { location });
    }

    let version = u32::from_le_bytes(data[4..8].try_into().unwrap());
    if version != GLB_VERSION_2 {
        return Err(ParseError::GlbUnsupportedVersion { version, location });
    }

    let mut offset = 12usize;
    let mut json_bytes: Option<&[u8]> = None;
    let mut bin_data: Option<Vec<u8>> = None;

    while offset + 8 <= data.len() {
        let chunk_length =
            u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        let chunk_type = u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap());
        let data_start = offset + 8;
        let data_end = data_start + chunk_length;

        if data_end > data.len() {
            return Err(ParseError::GlbChunkOutOfBounds { offset, location });
        }

        match chunk_type {
            CHUNK_TYPE_JSON if json_bytes.is_none() => {
                json_bytes = Some(&data[data_start..data_end]);
            }
            CHUNK_TYPE_BIN if bin_data.is_none() => {
                bin_data = Some(data[data_start..data_end].to_vec());
            }
            _ => {} // Unknown or duplicate chunks are ignored per spec.
        }

        offset = data_end;
    }

    let json_bytes = json_bytes.ok_or(ParseError::GlbMissingJsonChunk { location })?;

    let json_str =
        std::str::from_utf8(json_bytes).map_err(|_| ParseError::GlbJsonInvalidUtf8 { location })?;

    let gltf = parse(json_str)?;

    let buffers = match bin_data {
        Some(bin) => vec![bin],
        None => Vec::new(),
    };

    Ok((gltf, buffers))
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
