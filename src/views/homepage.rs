use conrod::*;

// widget_ids! {struct Ids {
//     text
// }}

pub struct HomepageView {
    // pub ids: Ids,
}

impl HomepageView {
    pub fn new(ui: &Ui) -> HomepageView {
        HomepageView {
            // ids: Ids::new(ui.widget_id_generator())
        }
    }

    pub fn display(ui: &UiBuilder) {
        ();
    }
}