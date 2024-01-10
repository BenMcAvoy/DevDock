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
   ( $key:expr => $value:expr ) => {{
       let map = hashmap!($key.to_string() => Some(vec![PortBinding {
           host_ip: None,
           host_port: Some($value.to_string())
       }]));

       dbg!(map)
   }}
}

#[macro_export]
macro_rules! envmap {
   ($($key:expr => $value:expr),+ $(,)?) => {{
       let mut env = Vec::new();
       $(
           env.push(format!("{}={}", $key, $value));
       )+
       env
   }}
}
