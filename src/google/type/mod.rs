// This file is @generated by prost-build.
/// Represents a day of the week.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DayOfWeek {
    /// The day of the week is unspecified.
    Unspecified = 0,
    /// Monday
    Monday = 1,
    /// Tuesday
    Tuesday = 2,
    /// Wednesday
    Wednesday = 3,
    /// Thursday
    Thursday = 4,
    /// Friday
    Friday = 5,
    /// Saturday
    Saturday = 6,
    /// Sunday
    Sunday = 7,
}
impl DayOfWeek {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "DAY_OF_WEEK_UNSPECIFIED",
            Self::Monday => "MONDAY",
            Self::Tuesday => "TUESDAY",
            Self::Wednesday => "WEDNESDAY",
            Self::Thursday => "THURSDAY",
            Self::Friday => "FRIDAY",
            Self::Saturday => "SATURDAY",
            Self::Sunday => "SUNDAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DAY_OF_WEEK_UNSPECIFIED" => Some(Self::Unspecified),
            "MONDAY" => Some(Self::Monday),
            "TUESDAY" => Some(Self::Tuesday),
            "WEDNESDAY" => Some(Self::Wednesday),
            "THURSDAY" => Some(Self::Thursday),
            "FRIDAY" => Some(Self::Friday),
            "SATURDAY" => Some(Self::Saturday),
            "SUNDAY" => Some(Self::Sunday),
            _ => None,
        }
    }
}
/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are [google.type.Date][google.type.Date] and
/// `google.protobuf.Timestamp`.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    #[prost(int32, tag = "1")]
    pub hours: i32,
    /// Minutes of hour of day. Must be from 0 to 59.
    #[prost(int32, tag = "2")]
    pub minutes: i32,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    #[prost(int32, tag = "3")]
    pub seconds: i32,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    #[prost(int32, tag = "4")]
    pub nanos: i32,
}
/// An object that represents a latitude/longitude pair. This is expressed as a
/// pair of doubles to represent degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="<http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84>
/// standard</a>. Values must be within normalized ranges.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range \[-90.0, +90.0\].
    #[prost(double, tag = "1")]
    pub latitude: f64,
    /// The longitude in degrees. It must be in the range \[-180.0, +180.0\].
    #[prost(double, tag = "2")]
    pub longitude: f64,
}
/// Represents a whole or partial calendar date, such as a birthday. The time of
/// day and time zone are either specified elsewhere or are insignificant. The
/// date is relative to the Gregorian Calendar. This can represent one of the
/// following:
///
/// * A full date, with non-zero year, month, and day values
/// * A month and day value, with a zero year, such as an anniversary
/// * A year on its own, with zero month and day values
/// * A year and month value, with a zero day, such as a credit card expiration
/// date
///
/// Related types are [google.type.TimeOfDay][google.type.TimeOfDay] and
/// `google.protobuf.Timestamp`.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Date {
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without
    /// a year.
    #[prost(int32, tag = "1")]
    pub year: i32,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a
    /// month and day.
    #[prost(int32, tag = "2")]
    pub month: i32,
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0
    /// to specify a year by itself or a year and month where the day isn't
    /// significant.
    #[prost(int32, tag = "3")]
    pub day: i32,
}
