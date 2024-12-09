pub struct FinderApi;

impl FinderApi {
    pub fn get_favorites_list(&self) -> Vec<String> {
        vec![String::from("/Applications")]
    }
}
