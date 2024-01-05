#[macro_export]
macro_rules! hashmap {
    () => (std::collection::HashMap::new());
    ($($key:expr => $value:expr), + $(,)?) => ({
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)+
        map
    })
}
