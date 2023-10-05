use thiserror::Error;

use crate::Selector;

#[derive(Error, Debug)]
pub enum MockingError {
    #[error("Message not found (unknown selector: {0:?})")]
    MessageNotFound(Selector),
    #[error("Decoding message arguments failed: {0:?}")]
    ArgumentDecoding(parity_scale_codec::Error),
}
