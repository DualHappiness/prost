#[derive(Default)]
#[serde(default)]
pub struct OutdirRequest {
        pub query: ::prost::alloc::string::String,
        pub page_number: i32,
        pub result_per_page: i32,
}
