use std::fmt;

use serde::{de, de::Visitor};
use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Clone, Debug, Deserialize, PartialEq, Hash, Eq)]
pub struct ObjectLayer {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub color: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub offsetx: i32,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub offsety: i32,
    #[serde(default = "topdown", rename = "draworder")]
    pub draw_order: String,
    #[serde(alias = "object")]
    pub objects: Vec<Object>,
}

fn topdown() -> String {
    "topdown".to_owned()
}

fn truth() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ObjectType {
    Ellipse,
    Point,
    Polygon {
        #[serde(deserialize_with = "deserialize_points_list")]
        points: Vec<(i32, i32)>,
    },
    Polyline {
        #[serde(deserialize_with = "deserialize_points_list")]
        points: Vec<(i32, i32)>,
    },
}

struct PointListVisitor;

impl<'de> Visitor<'de> for PointListVisitor {
    type Value = Vec<(i32, i32)>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format \"x,y x,y ...\"")
    }

    fn visit_str<E>(self, value: &str) -> Result<Vec<(i32, i32)>, E>
    where
        E: de::Error,
    {
        let mut failed = false;
        let v = value
            .split(' ')
            .map(|x| {
                let mut iter = x.split(',');
                let xs = iter.next();
                let ys = iter.next();
                if xs.is_none() || ys.is_none() {
                    failed = true;
                    return (0, 0);
                }
                let xr = xs.unwrap().parse::<f32>();
                let yr = ys.unwrap().parse::<f32>();
                if xr.is_err() || yr.is_err() {
                    failed = true;
                    return (0, 0);
                }
                (xr.unwrap().round() as i32, yr.unwrap().round() as i32)
            })
            .collect::<Vec<(i32, i32)>>();
        if failed {
            Err(de::Error::custom(""))
        } else {
            Ok(v)
        }
    }
}

fn deserialize_points_list<'de, D>(deserializer: D) -> Result<Vec<(i32, i32)>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_string(PointListVisitor)
}

pub fn deserialize_int_from_float_string<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Float(f32),
        Int(i32),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s
            .parse::<f32>()
            .map(|x| x.round() as i32)
            .map_err(serde::de::Error::custom),
        StringOrInt::Float(f) => Ok(f.round() as i32),
        StringOrInt::Int(i) => Ok(i),
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Object {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(deserialize_with = "deserialize_int_from_float_string")]
    pub x: i32,
    #[serde(deserialize_with = "deserialize_int_from_float_string")]
    pub y: i32,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub width: i32,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub height: i32,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub rotation: i32,
    #[serde(default = "truth", deserialize_with = "deserialize_bool_from_anything")]
    pub visible: bool,
}
