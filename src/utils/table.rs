/// Creates a table that can be rendered in GitHub markdown.
/// Example output:
/// <table><tr><td><td><td><tr><td>...</table>
pub fn create_markdown_table(titles: Vec<String>, mut data: Vec<Vec<String>>) -> String {
    data.insert(0, titles);

    format!(
        "<table>{}</table>",
        data.iter()
            .map(|v| {
                format!(
                    "<tr>{}",
                    v.iter()
                        .map(|v_inner| format!("<td>{}", v_inner))
                        .collect::<Vec<String>>()
                        .join("")
                )
            })
            .collect::<Vec<String>>()
            .join("")
    )
}
