use std::collections::HashMap;

use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
};

pub fn diff_to_table(hunk: String) -> String {
    let hunk_vec = hunk
        .split('\n')
        .filter_map(|s| parse_file_type(s))
        .collect::<Vec<&str>>();

    let mut file_type_counts: HashMap<String, usize> = HashMap::new();

    for item in hunk_vec.iter() {
        let counter = file_type_counts.entry(item.to_string()).or_insert(0);
        *counter += 1;
    }

    let mut table = term_table::Table::new();

    table.max_column_width = 10;
    table.style = term_table::TableStyle::rounded();

    for (k, v) in file_type_counts.iter() {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(k, 1, Alignment::Right),
            TableCell::new_with_alignment(v, 1, Alignment::Left),
        ]));
    }

    table.render()
}

fn parse_file_type(s: &str) -> Option<&str> {
    if s.starts_with("diff --git") {
        let current_file = s.split("b/").last().unwrap();

        if current_file.contains('.') {
            Some(current_file.split('.').last().unwrap())
        } else {
            None
        }
    } else {
        None
    }
}
