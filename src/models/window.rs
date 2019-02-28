use conrod::image::Id;

pub struct WindowModel {
    pub background: Option<Id>,
}

impl WindowModel {
    pub fn new() -> WindowModel {
        WindowModel {
            background: None,
        }
    }
}
