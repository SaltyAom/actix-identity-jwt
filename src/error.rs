pub mod http_error {
    #[inline(always)]
    pub async fn not_found() -> String {
        "Not found".to_owned()
    }
}
