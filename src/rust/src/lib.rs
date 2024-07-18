use extendr_api::prelude::*;

use crate::tbody::{Formatting, NAFormatting};
use std::io::Write;
mod header;
mod tbody;
mod tfoot;

fn list_from_vec_vec(values: Vec<Vec<&str>>, names: StrIter) -> List {
    let mut result_list = List::from_iter(values.into_iter().map(|mut values| {
        values.sort_unstable();
        values.dedup();
        values
    }));
    result_list
        .set_names(names)
        .expect("Must be able to set the names");
    result_list
}

#[extendr]
fn filter_from_values_vec(values_vec: Strings, data: List) -> List {
    let mut result_list = Vec::new();
    result_list.resize_with(data.len(), Vec::new);

    if values_vec.is_empty() {
        return list_from_vec_vec(result_list, data.names().expect("Must have names"));
    }

    let mut i = 0;
    while i < values_vec.len() {
        if let Some((x, y)) = values_vec[i].split_once(',') {
            let x: usize = x.parse().expect("X Must be a valid number");
            let y: usize = y.parse().expect("X Must be a valid number");
            if let Some(str_value) = data[x].index(y + 1).expect("Value must exists").as_str() {
                result_list[x].push(str_value);
            }
        }
        i += 1;
    }

    list_from_vec_vec(result_list, data.names().expect("Must have names"))
}

/// @export
#[extendr]
fn build_spyctable_html(
    data: List,
    names: Strings,
    nrow: i32,
    format: Formatting,
    na: NAFormatting,
    id: &str,
) -> String {
    let mut buffer = Vec::new();
    let _ = write!(
        &mut buffer,
        r#"<table id="{id}_inner_table" class="user-select-none table">"#
    );
    header::spyc_header_create(names, &mut buffer);
    tbody::build_tbody_and_foot(nrow, data, format, na, id, &mut buffer);
    let _ = write!(&mut buffer, "</table>");

    unsafe { String::from_utf8_unchecked(buffer) }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod spyctable;
    fn filter_from_values_vec;
    fn build_spyctable_html;
}
