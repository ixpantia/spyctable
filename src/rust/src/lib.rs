use extendr_api::prelude::*;
mod header;

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
fn filter_from_values_vec(values_vec: Integers, data: List) -> List {
    let mut result_list = Vec::new();
    result_list.resize_with(data.len(), Vec::new);

    if values_vec.is_empty() {
        return list_from_vec_vec(result_list, data.names().expect("Must have names"));
    }

    let mut i = 0;
    while i < values_vec.len() {
        let x = values_vec[i].inner() as usize;
        let y = values_vec[i + 1].inner() as usize;
        if let Some(str_value) = data[x].index(y + 1).expect("Value must exists").as_str() {
            result_list[x].push(str_value);
        }
        i += 2;
    }

    list_from_vec_vec(result_list, data.names().expect("Must have names"))
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod ixtable;
    use header;
    fn filter_from_values_vec;
}
