use chrono::NativeTimeStamp;

#[derive(Queryable)]
pub struct price {
    pub id: i32,
    pub ts: NativeTimeStamp,
    pub count: i32,
    pub percentile_5: i32,
    pub percentile_10: i32,
    pub percentile_20: i32,
    pub percentile_40: i32,
    pub percentile_80: i32,
}
