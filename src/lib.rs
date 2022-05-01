use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*};

pub struct TableTabs {
    column_names: Vec<String>,
    rows: Vec<Vec<Node>>,
}

impl TableTabs {
    pub fn new(column_names: Vec<String>) -> Self {
        Self {
            column_names,
            rows: vec![vec![]],
        }
    }

    pub fn add_row(&mut self, row: Vec<Node>) {
        if row.len() != self.column_names.len() {
            panic!("Must fill all column");
        }

        self.rows.push(row);
    }
}

impl NodeExt for TableTabs {
    fn into_node(self) -> Node {
        let mut table = Table.into_node();

        let mut header = Tr.into_node();

        for name in self.column_names {
            header.push_kid(Th.text(name));
        }
        table.push_kid(header);

        for nodes in self.rows {
            let mut row = Tr.into_node();

            for node in nodes {
                row.push_kid(Td::default().kid(node));
            }

            table.push_kid(row);
        }

        table
    }
}

/// Simple tabs
pub struct Tabs {
    name: String,
    tabs: Vec<(String, Node)>,
}

impl Tabs {
    pub fn new(name: String, tabs: Vec<(String, Node)>) -> Self {
        Self { name, tabs }
    }
}

impl NodeExt for Tabs {
    fn into_node(self) -> Node {
        // Start of the tabs as an unordered list.
        // let mut tabs = Ul.class("tabs");

        let mut tabs = Div.class("hs-tabs");

        // let id = &self.li_class;

        for (index, (name, contents)) in self.tabs.into_iter().enumerate() {
            let tab_id = &format!("hs-tab-{}-{index}", &self.name);

            let mut input = Input::radio("", &self.name);
            if index == 0 {
                input.set_checked();
            }
            let input = input.id(tab_id);

            let label = Label::new(tab_id).text(name);
            let contents = Div.kid(contents).class("contents scrollbar");

            tabs.push_kid(input);
            tabs.push_kid(label);
            tabs.push_kid(contents);

            // let unique = format!("{id}-{index}");
            // let input_id = format!("tab-{unique}");

            // let mut input = Input::radio("", &self.name);
            // if index == 0 {
            //     input.set_checked();
            // }
            // let input = input.id(&input_id);

            // tabs.push_kid(
            //     Li.class("tab")
            //         .kid(input)
            //         .kid(Label::new(&input_id).text(name))
            //         .kid(
            //             Div.class("content")
            //                 .id(format!("tab-content-{unique}"))
            //                 .kid(contents),
            //         ),
            // );
        }

        tabs
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn tabs() {
//         let tabs = Tabs {
//             id: "Tabby".into(),
//             tabs: vec![
//                 ("A".into(), P.text("Paragraph A")),
//                 ("B".into(), P.text("Paragraph B")),
//                 ("C".into(), P.text("Paragraph C")),
//             ],
//         };

//         tabs.into_node()
//             .render_writer(&mut std::fs::File::create("tabs.html").unwrap())
//             .unwrap();
//     }
// }
