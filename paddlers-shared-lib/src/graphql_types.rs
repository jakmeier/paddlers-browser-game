#[cfg(feature = "graphql")] 
use juniper::Value;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct GqlTimestamp(pub i64);
use chrono::offset::TimeZone;
impl GqlTimestamp {
    pub fn from_string(s: &String) -> Option<GqlTimestamp> {
        s.parse::<i64>().ok().map(GqlTimestamp)
    }
    pub fn from_chrono(ndt: &chrono::NaiveDateTime) -> GqlTimestamp {
        // let date = chrono::Utc.from_local_datetime(ndt);
        //     if date.single().is_none() {
        //     // return Err("Datetime from DB is not unique".into());
        //     panic!("asdf");
        // }
        // let ndt = date.unwrap().naive_utc();
        GqlTimestamp(
            ndt.timestamp() * 1_000_000 + ndt.timestamp_subsec_micros() as i64
        )
    }
    pub fn to_chrono(&self) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::from_timestamp(self.0 / 1_000_000, (self.0 % 1_000_000) as u32)
    }
}

#[cfg(feature = "graphql")] 
juniper::graphql_scalar!(GqlTimestamp {
    description: "Micro second precision timestamp"

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<GqlTimestamp> {
        v.as_scalar_value::<String>()
            .and_then(GqlTimestamp::from_string)
    }

    from_str<'a>(value: ScalarToken<'a>) -> juniper::ParseScalarResult<'a, juniper::DefaultScalarValue> {
        <String as juniper::ParseScalarValue>::from_str(value)
    }
});