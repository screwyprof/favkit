mod errors;

use errors::Result;
pub struct FinderApi;

impl FinderApi {
    pub fn get_favorites_list(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_favorites_list_returns_empty_vector() {
        let api = FinderApi;
        let result = api.get_favorites_list();
        assert!(result.is_ok());
    }
}
