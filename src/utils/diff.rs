use std::collections::HashMap;

use crate::handlers::files::FileData;

/// Getting stats on different types of files that have been changed.
/// Documentation how diffs are formatted:
/// https://git-scm.com/docs/git-diff#_combined_diff_format
pub fn parse_diff(hunk: String) -> Vec<FileData> {
    let vec_hunk = hunk.split('\n').collect::<Vec<&str>>();

    let mut data: HashMap<Option<String>, FileData> = HashMap::new();

    for i in 0..=vec_hunk.len() {
        let current_item = vec_hunk[i];

        if current_item.starts_with("diff --git") {
            let file_type: Option<String> = if current_item.contains('.') {
                Some(current_item.split('.').last().unwrap().to_string())
            } else {
                None
            };

            let file_data = FileData::new(
                file_type.clone(),
                1,
                if vec_hunk[i + 1].starts_with("new file mode") {
                    1
                } else {
                    0
                },
                if vec_hunk[i + 1].starts_with("deleted file mode") {
                    1
                } else {
                    0
                },
                0,
                0,
            );

            if data.contains_key(&file_type) {
                let mut single_value = data.get_mut(&file_type).unwrap();

                single_value += file_data;
            }
        }
    }

    let mut data_values = data.into_values().collect::<Vec<FileData>>();

    data_values.sort_by(|a, b| b.amount.cmp(&a.amount));

    data_values
}
