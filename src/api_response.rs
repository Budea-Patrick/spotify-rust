use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

impl<T> Items<T> {
    pub(crate) fn get_items(self) -> Vec<T> {
        return self.items;
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct APIResponse<T> {
    items: Items<T>,
}

impl<T> APIResponse<T> {
    pub(crate) fn get_items(self) -> Items<T> {
        return self.items;
    }
}
