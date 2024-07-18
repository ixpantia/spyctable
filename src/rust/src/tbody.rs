use extendr_api::prelude::*;
use extendr_api::AsTypedSlice;
use std::io::Write;

#[derive(Copy, Clone)]
pub enum Formatting {
    Millions,
    Thousands,
    Default,
}

impl<'a> FromRobj<'a> for Formatting {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(val) = robj.as_str() {
            return Ok(match val {
                "MM" => Formatting::Millions,
                "K" => Formatting::Thousands,
                "default" => Formatting::Default,
                _ => panic!("Invalid formatting"),
            });
        }
        Ok(Formatting::Default)
    }
}

#[derive(Copy, Clone)]
pub enum NAFormatting {
    Dash,
    Zero,
}

impl<'a> FromRobj<'a> for NAFormatting {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(val) = robj.as_str() {
            return Ok(match val {
                "dash" => NAFormatting::Dash,
                "zero" => NAFormatting::Zero,
                _ => panic!("Invalid NA formatting"),
            });
        }
        Ok(NAFormatting::Zero)
    }
}

fn build_td_na(na: NAFormatting, buffer: &mut Vec<u8>) {
    let _ = match na {
        NAFormatting::Dash => write!(
            buffer,
            r#"<td class="border text-center align-middle user-select-none">-</td>"#
        ),
        NAFormatting::Zero => write!(
            buffer,
            r#"<td class="border text-center align-middle user-select-none">0</td>"#
        ),
    };
}

fn build_td_non_na(format: Formatting, val: f64, buffer: &mut Vec<u8>) {
    let _ = match format {
        Formatting::Default => write!(
            buffer,
            r#"<td class="border text-center align-middle user-select-none">{val:.2}</td>"#
        ),
        Formatting::Millions => write!(
            buffer,
            r#"<td class="border text-center align-middle user-select-none">{val:.0}MM</td>"#,
            val = val / 1_000_000.0
        ),
        Formatting::Thousands => write!(
            buffer,
            r#"<td class="border text-center align-middle user-select-none">{val:.0}K</td>"#,
            val = val / 1_000.0
        ),
    };
}

pub fn build_tbody_and_foot(
    nrow: i32,
    data: List,
    format: Formatting,
    na: NAFormatting,
    table_id: &str,
    buffer: &mut Vec<u8>,
) {
    let nrow = nrow as usize;
    let is_real_col_lookup: Vec<bool> = data.iter().map(|(_, col)| col.is_real()).collect();
    let mut total_sum_lookup: Vec<f64> = is_real_col_lookup
        .iter()
        .map(|is_real| if *is_real { 0.0 } else { -1.0 })
        .collect();
    let ncol = is_real_col_lookup.len();

    let _ = write!(
        buffer,
        r#"<tbody onmousedown="enable_dragging()" onmouseleave="disable_dragging()" onmouseenter="disable_dragging()">"#
    );

    (0..nrow).for_each(|i| {
        let _ = write!(buffer, "<tr>");
        for c in 0..ncol {
            if is_real_col_lookup[c] {
                if let Some(real_slice) = data[c].as_real_slice() {
                    let val = real_slice[i];
                    if val.is_na() {
                        build_td_na(na, buffer);
                    } else {
                        total_sum_lookup[c] += val;
                        build_td_non_na(format, val, buffer);
                    }
                }
            } else {
                let _ = match AsTypedSlice::<'_, Rstr>::as_typed_slice(&data[c]) {
                    Some(str_slice) => {
                        let val = &str_slice[i];
                        write!(
                            buffer,
                            r#"
                            <td data-coords="{c},{i}" data-table_id="{table_id}" class="user-select-none border text-center align-middle" onmouseover="mouse_over_event(this)" onmousedown="mouse_down_event(this)">{val}</td>
                            "#
                        )
                    }
                    None => write!(
                        buffer,
                        r#"<td data-coords="{c},{i}" data-table_id="{table_id}" class="user-select-none border text-center align-middle" onmouseover="mouse_over_event(this)" onmousedown="mouse_down_event(this)"></td>"#
                    ),
                };
            }
        }
        let _ = write!(buffer, "</tr>");
    });

    let _ = write!(buffer, "</tbody>");
    let _ = write!(buffer, "<tfoot>");

    let _ = write!(buffer, "<tr>");
    let _ = write!(
        buffer,
        r#"<td class="border text-center align-middle">Total</td>"#
    );
    (1..ncol).for_each(|c| {
        let val = total_sum_lookup[c];
        if val == -1.0 {
            let _ = write!(
                buffer,
                r#"<td class="border text-center align-middle"></td>"#
            );
        } else {
            build_td_non_na(format, val, buffer);
        }
    });
    let _ = write!(buffer, "</tr>");

    let _ = write!(buffer, "<tr>");
    let _ = write!(
        buffer,
        r#"<td class="border text-center align-middle">Average</td>"#
    );
    (1..ncol).for_each(|c| {
        let val = total_sum_lookup[c];
        if val == -1.0 {
            let _ = write!(
                buffer,
                r#"<td class="border text-center align-middle"></td>"#
            );
        } else {
            build_td_non_na(format, val / nrow as f64, buffer);
        }
    });
    let _ = write!(buffer, "</tr>");

    let _ = write!(buffer, "</tfoot>");
}

extendr_module! {
    mod tbody;
}
