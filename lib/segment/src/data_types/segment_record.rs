use smallvec::SmallVec;

use crate::data_types::vectors::VectorInternal;
use crate::types::{Payload, PointIdType, RawPayload, VectorNameBuf};

/// A point almost always has a single (default) named vector, so keep it inline
/// to avoid a heap allocation on the common retrieve path.
pub type NamedVectorsOwned = SmallVec<[(VectorNameBuf, VectorInternal); 1]>;

/// Byte-blob analogue of [`NamedVectorsOwned`]: vectors as storage-native bytes.
pub type NamedVectorBytesOwned = SmallVec<[(VectorNameBuf, Vec<u8>); 1]>;

/// A retrieved point: id, optional vectors, optional payload.
pub struct SegmentRecord {
    pub id: PointIdType,
    pub vectors: Option<NamedVectorsOwned>,
    pub payload: Option<Payload>,
}

/// Byte-blob analogue of [`SegmentRecord`].
///
/// Which of the two payload fields is filled is decided by the
/// [`RawPayloadFormat`] the retrieval was asked for; they are never both set.
pub struct SegmentRecordRaw {
    pub id: PointIdType,
    pub vectors: Option<NamedVectorBytesOwned>,
    pub payload: Option<Payload>,
    pub payload_raw: Option<RawPayload>,
}

/// How a raw retrieval should read the payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RawPayloadFormat {
    /// Parse into a [`Payload`], like the non-raw retrieve does. Fills
    /// [`SegmentRecordRaw::payload`].
    #[default]
    Parsed,
    /// Hand back the stored byte blob, skipping the parse. Fills
    /// [`SegmentRecordRaw::payload_raw`].
    ///
    /// Only possible without a payload selector: a selection cannot be applied
    /// to an opaque blob, so a selector falls back to [`Self::Parsed`].
    Bytes,
}
