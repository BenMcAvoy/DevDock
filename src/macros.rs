#[macro_export]
macro_rules! hashmap {
    () => (std::collection::HashMap::new());
    ($($key:expr => $value:expr), + $(,)?) => ({
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)+
        map
    })
}

#[macro_export]
macro_rules! mapping {
   ( $key:expr => $val:expr ) => {{
       hashmap!($key.to_string() => Some(vec![PortBinding {
           host_ip: None,
           host_port: Some($key.to_string())
       }]))
   }}
}
