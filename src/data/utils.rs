use serde::de::DeserializeOwned;

pub fn parse_csv<T>(str: String) -> Vec<T>
where
    T: DeserializeOwned,
{
    csv::Reader::from_reader(str.as_bytes())
        .deserialize::<T>()
        .map(|i| i.unwrap())
        .collect()
}
