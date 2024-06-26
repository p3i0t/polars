use arrow::array::{Array, BinaryViewArray};
use polars_error::PolarsResult;

use super::super::{nested, utils, WriteOptions};
use super::basic::{build_statistics, encode_plain};
use crate::arrow::write::Nested;
use crate::parquet::encoding::Encoding;
use crate::parquet::page::DataPage;
use crate::parquet::schema::types::PrimitiveType;

pub fn array_to_page(
    array: &BinaryViewArray,
    options: WriteOptions,
    type_: PrimitiveType,
    nested: &[Nested],
) -> PolarsResult<DataPage> {
    let mut buffer = vec![];
    let (repetition_levels_byte_length, definition_levels_byte_length) =
        nested::write_rep_and_def(options.version, nested, &mut buffer)?;

    encode_plain(array, &mut buffer);

    let statistics = if options.has_statistics() {
        Some(build_statistics(array, type_.clone(), &options.statistics))
    } else {
        None
    };

    utils::build_plain_page(
        buffer,
        nested::num_values(nested),
        nested[0].len(),
        array.null_count(),
        repetition_levels_byte_length,
        definition_levels_byte_length,
        statistics,
        type_,
        options,
        Encoding::Plain,
    )
}
