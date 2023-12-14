use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs, Value};
use uuid::Uuid;

/// Primary Redis key
pub struct EntityKey(pub Uuid);

impl ToRedisArgs for EntityKey {
    fn write_redis_args<W>(&self, out: &mut W) where W: ?Sized + RedisWrite {
        out.write_arg(self.0.as_bytes())
    }
}

impl TryFrom<&[u8]> for EntityKey {
    type Error = RedisError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let v = value.try_into().map_err(|_| make_err(ErrorKind::ResponseError, "Invalid entity key"))?;
        Ok(Self(Uuid::from_bytes(v)))
    }
}

impl FromRedisValue for EntityKey {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Data(d) => Self::try_from(d.as_slice()),
            _ => Err(make_err(ErrorKind::TypeError, "EntityKey can only be constructed from Data type"))
        }
    }

    fn from_byte_vec(v: &[u8]) -> Option<Vec<Self>> {
        Some(vec![Self::try_from(v).ok()?])
    }
}

/// Redis Hash key for each type of component
pub struct ComponentKey(pub u16);

impl ToRedisArgs for ComponentKey {
    fn write_redis_args<W>(&self, out: &mut W) where W: ?Sized + RedisWrite {
        out.write_arg(&self.0.to_le_bytes())
    }
}

impl TryFrom<&[u8]> for ComponentKey {
    type Error = RedisError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let bytes = value.try_into()
            .map_err(|_| make_err(ErrorKind::ResponseError, "Invalid ComponentKey"))?;
        Ok(Self(u16::from_le_bytes(bytes)))
    }
}

impl FromRedisValue for ComponentKey {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Int(i) => {
                if *i < 0 || *i > u16::MAX as i64 {
                    return Err(make_err(ErrorKind::ResponseError, "ComponentKey out of range"));
                }
                let i = *i as u16;
                Ok(Self(i))
            }
            Value::Data(d) => Self::try_from(d.as_slice()),
            _ => Err(make_err(ErrorKind::TypeError, "ComponentKey can only be constructed from Data or Int type"))
        }
    }

    fn from_byte_vec(v: &[u8]) -> Option<Vec<Self>> {
        Some(vec![Self::try_from(v).ok()?])
    }
}

fn make_err(kind: ErrorKind, s: &'static str) -> RedisError {
    RedisError::from((kind, s))
}