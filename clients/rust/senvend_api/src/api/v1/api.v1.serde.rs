// @generated
impl serde::Serialize for AgeApiFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeApiFailure", len)?;
        if self.reason != 0 {
            let v = AgeApiFailureReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeApiFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeApiFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeApiFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeApiFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<AgeApiFailureReason>()? as i32);
                        }
                    }
                }
                Ok(AgeApiFailure {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeApiFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeApiFailureReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AGE_API_FAILURE_REASON_UNSPECIFIED",
            Self::InvalidAge => "AGE_API_FAILURE_REASON_INVALID_AGE",
            Self::UuidNotFound => "AGE_API_FAILURE_REASON_UUID_NOT_FOUND",
            Self::UnknownCommand => "AGE_API_FAILURE_REASON_UNKNOWN_COMMAND",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AgeApiFailureReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AGE_API_FAILURE_REASON_UNSPECIFIED",
            "AGE_API_FAILURE_REASON_INVALID_AGE",
            "AGE_API_FAILURE_REASON_UUID_NOT_FOUND",
            "AGE_API_FAILURE_REASON_UNKNOWN_COMMAND",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeApiFailureReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AGE_API_FAILURE_REASON_UNSPECIFIED" => Ok(AgeApiFailureReason::Unspecified),
                    "AGE_API_FAILURE_REASON_INVALID_AGE" => Ok(AgeApiFailureReason::InvalidAge),
                    "AGE_API_FAILURE_REASON_UUID_NOT_FOUND" => Ok(AgeApiFailureReason::UuidNotFound),
                    "AGE_API_FAILURE_REASON_UNKNOWN_COMMAND" => Ok(AgeApiFailureReason::UnknownCommand),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgeApiSuccess {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeApiSuccess", len)?;
        if self.reason != 0 {
            let v = AgeApiSuccessReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeApiSuccess {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeApiSuccess;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeApiSuccess")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeApiSuccess, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<AgeApiSuccessReason>()? as i32);
                        }
                    }
                }
                Ok(AgeApiSuccess {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeApiSuccess", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeApiSuccessReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AGE_API_SUCCESS_REASON_UNSPECIFIED",
            Self::VerificationStarted => "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED",
            Self::CancelAccepted => "AGE_API_SUCCESS_REASON_CANCEL_ACCEPTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AgeApiSuccessReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AGE_API_SUCCESS_REASON_UNSPECIFIED",
            "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED",
            "AGE_API_SUCCESS_REASON_CANCEL_ACCEPTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeApiSuccessReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AGE_API_SUCCESS_REASON_UNSPECIFIED" => Ok(AgeApiSuccessReason::Unspecified),
                    "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" => Ok(AgeApiSuccessReason::VerificationStarted),
                    "AGE_API_SUCCESS_REASON_CANCEL_ACCEPTED" => Ok(AgeApiSuccessReason::CancelAccepted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgeCancelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.AgeCancelRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeCancelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeCancelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeCancelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeCancelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AgeCancelRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeCancelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeFailure", len)?;
        if let Some(v) = self.reason.as_ref() {
            match v {
                age_failure::Reason::FailureReason(v) => {
                    let v = AgeFailureReason::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("failureReason", &v)?;
                }
                age_failure::Reason::UnderAge(v) => {
                    struct_ser.serialize_field("underAge", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failure_reason",
            "failureReason",
            "under_age",
            "underAge",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailureReason,
            UnderAge,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "failureReason" | "failure_reason" => Ok(GeneratedField::FailureReason),
                            "underAge" | "under_age" => Ok(GeneratedField::UnderAge),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FailureReason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureReason"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<AgeFailureReason>>()?.map(|x| age_failure::Reason::FailureReason(x as i32));
                        }
                        GeneratedField::UnderAge => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("underAge"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(age_failure::Reason::UnderAge)
;
                        }
                    }
                }
                Ok(AgeFailure {
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeFailureReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AGE_FAILURE_REASON_UNSPECIFIED",
            Self::UserCancelled => "AGE_FAILURE_REASON_USER_CANCELLED",
            Self::Timeout => "AGE_FAILURE_REASON_TIMEOUT",
            Self::VerificationOngoing => "AGE_FAILURE_REASON_VERIFICATION_ONGOING",
            Self::InvalidState => "AGE_FAILURE_REASON_INVALID_STATE",
            Self::ApiCancelled => "AGE_FAILURE_REASON_API_CANCELLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AgeFailureReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AGE_FAILURE_REASON_UNSPECIFIED",
            "AGE_FAILURE_REASON_USER_CANCELLED",
            "AGE_FAILURE_REASON_TIMEOUT",
            "AGE_FAILURE_REASON_VERIFICATION_ONGOING",
            "AGE_FAILURE_REASON_INVALID_STATE",
            "AGE_FAILURE_REASON_API_CANCELLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeFailureReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AGE_FAILURE_REASON_UNSPECIFIED" => Ok(AgeFailureReason::Unspecified),
                    "AGE_FAILURE_REASON_USER_CANCELLED" => Ok(AgeFailureReason::UserCancelled),
                    "AGE_FAILURE_REASON_TIMEOUT" => Ok(AgeFailureReason::Timeout),
                    "AGE_FAILURE_REASON_VERIFICATION_ONGOING" => Ok(AgeFailureReason::VerificationOngoing),
                    "AGE_FAILURE_REASON_INVALID_STATE" => Ok(AgeFailureReason::InvalidState),
                    "AGE_FAILURE_REASON_API_CANCELLED" => Ok(AgeFailureReason::ApiCancelled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgeFailureUnderage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.determined_age != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeFailureUnderage", len)?;
        if self.determined_age != 0 {
            struct_ser.serialize_field("determinedAge", &self.determined_age)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeFailureUnderage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "determined_age",
            "determinedAge",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeterminedAge,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "determinedAge" | "determined_age" => Ok(GeneratedField::DeterminedAge),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeFailureUnderage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeFailureUnderage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeFailureUnderage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut determined_age__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeterminedAge => {
                            if determined_age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("determinedAge"));
                            }
                            determined_age__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AgeFailureUnderage {
                    determined_age: determined_age__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeFailureUnderage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeRequest", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                age_request::Request::Start(v) => {
                    struct_ser.serialize_field("start", v)?;
                }
                age_request::Request::Cancel(v) => {
                    struct_ser.serialize_field("cancel", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "start",
            "cancel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Start,
            Cancel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "start" => Ok(GeneratedField::Start),
                            "cancel" => Ok(GeneratedField::Cancel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Start => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(age_request::Request::Start)
;
                        }
                        GeneratedField::Cancel => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancel"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(age_request::Request::Cancel)
;
                        }
                    }
                }
                Ok(AgeRequest {
                    id: id__,
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeResponse", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            match v {
                age_response::Result::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                age_response::Result::Failure(v) => {
                    struct_ser.serialize_field("failure", v)?;
                }
                age_response::Result::ApiSuccess(v) => {
                    struct_ser.serialize_field("apiSuccess", v)?;
                }
                age_response::Result::ApiFailure(v) => {
                    struct_ser.serialize_field("apiFailure", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "success",
            "failure",
            "api_success",
            "apiSuccess",
            "api_failure",
            "apiFailure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Success,
            Failure,
            ApiSuccess,
            ApiFailure,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "success" => Ok(GeneratedField::Success),
                            "failure" => Ok(GeneratedField::Failure),
                            "apiSuccess" | "api_success" => Ok(GeneratedField::ApiSuccess),
                            "apiFailure" | "api_failure" => Ok(GeneratedField::ApiFailure),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Success => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(age_response::Result::Success)
;
                        }
                        GeneratedField::Failure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(age_response::Result::Failure)
;
                        }
                        GeneratedField::ApiSuccess => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiSuccess"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(age_response::Result::ApiSuccess)
;
                        }
                        GeneratedField::ApiFailure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiFailure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(age_response::Result::ApiFailure)
;
                        }
                    }
                }
                Ok(AgeResponse {
                    id: id__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeStartRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_age != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeStartRequest", len)?;
        if self.min_age != 0 {
            struct_ser.serialize_field("minAge", &self.min_age)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeStartRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_age",
            "minAge",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinAge,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minAge" | "min_age" => Ok(GeneratedField::MinAge),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeStartRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeStartRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeStartRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_age__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinAge => {
                            if min_age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAge"));
                            }
                            min_age__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AgeStartRequest {
                    min_age: min_age__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeStartRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgeSuccess {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.AgeSuccess", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeSuccess {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgeSuccess;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeSuccess")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeSuccess, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AgeSuccess {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeSuccess", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayApiFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayApiFailure", len)?;
        if self.reason != 0 {
            let v = PayApiFailureReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayApiFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayApiFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayApiFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayApiFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<PayApiFailureReason>()? as i32);
                        }
                    }
                }
                Ok(PayApiFailure {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayApiFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayApiFailureReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PAY_API_FAILURE_REASON_UNSPECIFIED",
            Self::InvalidAmount => "PAY_API_FAILURE_REASON_INVALID_AMOUNT",
            Self::UuidNotFound => "PAY_API_FAILURE_REASON_UUID_NOT_FOUND",
            Self::UnknownCommand => "PAY_API_FAILURE_REASON_UNKNOWN_COMMAND",
            Self::NoApprovedPayment => "PAY_API_FAILURE_REASON_NO_APPROVED_PAYMENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PayApiFailureReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PAY_API_FAILURE_REASON_UNSPECIFIED",
            "PAY_API_FAILURE_REASON_INVALID_AMOUNT",
            "PAY_API_FAILURE_REASON_UUID_NOT_FOUND",
            "PAY_API_FAILURE_REASON_UNKNOWN_COMMAND",
            "PAY_API_FAILURE_REASON_NO_APPROVED_PAYMENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayApiFailureReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PAY_API_FAILURE_REASON_UNSPECIFIED" => Ok(PayApiFailureReason::Unspecified),
                    "PAY_API_FAILURE_REASON_INVALID_AMOUNT" => Ok(PayApiFailureReason::InvalidAmount),
                    "PAY_API_FAILURE_REASON_UUID_NOT_FOUND" => Ok(PayApiFailureReason::UuidNotFound),
                    "PAY_API_FAILURE_REASON_UNKNOWN_COMMAND" => Ok(PayApiFailureReason::UnknownCommand),
                    "PAY_API_FAILURE_REASON_NO_APPROVED_PAYMENT" => Ok(PayApiFailureReason::NoApprovedPayment),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PayApiSuccess {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayApiSuccess", len)?;
        if self.reason != 0 {
            let v = PayApiSuccessReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayApiSuccess {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayApiSuccess;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayApiSuccess")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayApiSuccess, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<PayApiSuccessReason>()? as i32);
                        }
                    }
                }
                Ok(PayApiSuccess {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayApiSuccess", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayApiSuccessReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PAY_API_SUCCESS_REASON_UNSPECIFIED",
            Self::PaymentStarted => "PAY_API_SUCCESS_REASON_PAYMENT_STARTED",
            Self::CancelAccepted => "PAY_API_SUCCESS_REASON_CANCEL_ACCEPTED",
            Self::GoodsIssuedAccepted => "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PayApiSuccessReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PAY_API_SUCCESS_REASON_UNSPECIFIED",
            "PAY_API_SUCCESS_REASON_PAYMENT_STARTED",
            "PAY_API_SUCCESS_REASON_CANCEL_ACCEPTED",
            "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayApiSuccessReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PAY_API_SUCCESS_REASON_UNSPECIFIED" => Ok(PayApiSuccessReason::Unspecified),
                    "PAY_API_SUCCESS_REASON_PAYMENT_STARTED" => Ok(PayApiSuccessReason::PaymentStarted),
                    "PAY_API_SUCCESS_REASON_CANCEL_ACCEPTED" => Ok(PayApiSuccessReason::CancelAccepted),
                    "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED" => Ok(PayApiSuccessReason::GoodsIssuedAccepted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PayApproved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.PayApproved", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayApproved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayApproved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayApproved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayApproved, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PayApproved {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayApproved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayCancel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.PayCancel", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayCancel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayCancel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PayCancel {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayCancel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayFailure", len)?;
        if let Some(v) = self.reason.as_ref() {
            match v {
                pay_failure::Reason::FailureReason(v) => {
                    let v = PayFailureReason::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("failureReason", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failure_reason",
            "failureReason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailureReason,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "failureReason" | "failure_reason" => Ok(GeneratedField::FailureReason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FailureReason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureReason"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<PayFailureReason>>()?.map(|x| pay_failure::Reason::FailureReason(x as i32));
                        }
                    }
                }
                Ok(PayFailure {
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayFailureReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PAY_FAILURE_REASON_UNSPECIFIED",
            Self::UserCancelled => "PAY_FAILURE_REASON_USER_CANCELLED",
            Self::Timeout => "PAY_FAILURE_REASON_TIMEOUT",
            Self::PaymentOngoing => "PAY_FAILURE_REASON_PAYMENT_ONGOING",
            Self::InvalidState => "PAY_FAILURE_REASON_INVALID_STATE",
            Self::PaymentFailed => "PAY_FAILURE_REASON_PAYMENT_FAILED",
            Self::ApiCancelled => "PAY_FAILURE_REASON_API_CANCELLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PayFailureReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PAY_FAILURE_REASON_UNSPECIFIED",
            "PAY_FAILURE_REASON_USER_CANCELLED",
            "PAY_FAILURE_REASON_TIMEOUT",
            "PAY_FAILURE_REASON_PAYMENT_ONGOING",
            "PAY_FAILURE_REASON_INVALID_STATE",
            "PAY_FAILURE_REASON_PAYMENT_FAILED",
            "PAY_FAILURE_REASON_API_CANCELLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayFailureReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PAY_FAILURE_REASON_UNSPECIFIED" => Ok(PayFailureReason::Unspecified),
                    "PAY_FAILURE_REASON_USER_CANCELLED" => Ok(PayFailureReason::UserCancelled),
                    "PAY_FAILURE_REASON_TIMEOUT" => Ok(PayFailureReason::Timeout),
                    "PAY_FAILURE_REASON_PAYMENT_ONGOING" => Ok(PayFailureReason::PaymentOngoing),
                    "PAY_FAILURE_REASON_INVALID_STATE" => Ok(PayFailureReason::InvalidState),
                    "PAY_FAILURE_REASON_PAYMENT_FAILED" => Ok(PayFailureReason::PaymentFailed),
                    "PAY_FAILURE_REASON_API_CANCELLED" => Ok(PayFailureReason::ApiCancelled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PayGoodsIssued {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.partial_amount != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayGoodsIssued", len)?;
        if self.partial_amount != 0 {
            struct_ser.serialize_field("partialAmount", &self.partial_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayGoodsIssued {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partial_amount",
            "partialAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartialAmount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "partialAmount" | "partial_amount" => Ok(GeneratedField::PartialAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayGoodsIssued;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayGoodsIssued")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayGoodsIssued, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partial_amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PartialAmount => {
                            if partial_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partialAmount"));
                            }
                            partial_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PayGoodsIssued {
                    partial_amount: partial_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayGoodsIssued", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayRequest", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                pay_request::Request::Start(v) => {
                    struct_ser.serialize_field("start", v)?;
                }
                pay_request::Request::Cancel(v) => {
                    struct_ser.serialize_field("cancel", v)?;
                }
                pay_request::Request::GoodsIssued(v) => {
                    struct_ser.serialize_field("goodsIssued", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "start",
            "cancel",
            "goods_issued",
            "goodsIssued",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Start,
            Cancel,
            GoodsIssued,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "start" => Ok(GeneratedField::Start),
                            "cancel" => Ok(GeneratedField::Cancel),
                            "goodsIssued" | "goods_issued" => Ok(GeneratedField::GoodsIssued),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Start => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_request::Request::Start)
;
                        }
                        GeneratedField::Cancel => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancel"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_request::Request::Cancel)
;
                        }
                        GeneratedField::GoodsIssued => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("goodsIssued"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_request::Request::GoodsIssued)
;
                        }
                    }
                }
                Ok(PayRequest {
                    id: id__,
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayResponse", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            match v {
                pay_response::Result::Approved(v) => {
                    struct_ser.serialize_field("approved", v)?;
                }
                pay_response::Result::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                pay_response::Result::Failure(v) => {
                    struct_ser.serialize_field("failure", v)?;
                }
                pay_response::Result::ApiFailure(v) => {
                    struct_ser.serialize_field("apiFailure", v)?;
                }
                pay_response::Result::ApiSuccess(v) => {
                    struct_ser.serialize_field("apiSuccess", v)?;
                }
                pay_response::Result::AgeSuccess(v) => {
                    struct_ser.serialize_field("ageSuccess", v)?;
                }
                pay_response::Result::AgeFailure(v) => {
                    struct_ser.serialize_field("ageFailure", v)?;
                }
                pay_response::Result::AgeApiSuccess(v) => {
                    struct_ser.serialize_field("ageApiSuccess", v)?;
                }
                pay_response::Result::AgeApiFailure(v) => {
                    struct_ser.serialize_field("ageApiFailure", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "approved",
            "success",
            "failure",
            "api_failure",
            "apiFailure",
            "api_success",
            "apiSuccess",
            "age_success",
            "ageSuccess",
            "age_failure",
            "ageFailure",
            "age_api_success",
            "ageApiSuccess",
            "age_api_failure",
            "ageApiFailure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Approved,
            Success,
            Failure,
            ApiFailure,
            ApiSuccess,
            AgeSuccess,
            AgeFailure,
            AgeApiSuccess,
            AgeApiFailure,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "approved" => Ok(GeneratedField::Approved),
                            "success" => Ok(GeneratedField::Success),
                            "failure" => Ok(GeneratedField::Failure),
                            "apiFailure" | "api_failure" => Ok(GeneratedField::ApiFailure),
                            "apiSuccess" | "api_success" => Ok(GeneratedField::ApiSuccess),
                            "ageSuccess" | "age_success" => Ok(GeneratedField::AgeSuccess),
                            "ageFailure" | "age_failure" => Ok(GeneratedField::AgeFailure),
                            "ageApiSuccess" | "age_api_success" => Ok(GeneratedField::AgeApiSuccess),
                            "ageApiFailure" | "age_api_failure" => Ok(GeneratedField::AgeApiFailure),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Approved => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("approved"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::Approved)
;
                        }
                        GeneratedField::Success => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::Success)
;
                        }
                        GeneratedField::Failure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::Failure)
;
                        }
                        GeneratedField::ApiFailure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiFailure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::ApiFailure)
;
                        }
                        GeneratedField::ApiSuccess => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiSuccess"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::ApiSuccess)
;
                        }
                        GeneratedField::AgeSuccess => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageSuccess"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::AgeSuccess)
;
                        }
                        GeneratedField::AgeFailure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageFailure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::AgeFailure)
;
                        }
                        GeneratedField::AgeApiSuccess => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageApiSuccess"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::AgeApiSuccess)
;
                        }
                        GeneratedField::AgeApiFailure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageApiFailure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::AgeApiFailure)
;
                        }
                    }
                }
                Ok(PayResponse {
                    id: id__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PayStart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount != 0 {
            len += 1;
        }
        if self.age_verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayStart", len)?;
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if let Some(v) = self.age_verification.as_ref() {
            struct_ser.serialize_field("ageVerification", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayStart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
            "age_verification",
            "ageVerification",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            AgeVerification,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "amount" => Ok(GeneratedField::Amount),
                            "ageVerification" | "age_verification" => Ok(GeneratedField::AgeVerification),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayStart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayStart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayStart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                let mut age_verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AgeVerification => {
                            if age_verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageVerification"));
                            }
                            age_verification__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PayStart {
                    amount: amount__.unwrap_or_default(),
                    age_verification: age_verification__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayStart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaySuccess {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.PaySuccess", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaySuccess {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaySuccess;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PaySuccess")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PaySuccess, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PaySuccess {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PaySuccess", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Uuid4 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msb != 0 {
            len += 1;
        }
        if self.lsb != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.Uuid4", len)?;
        if self.msb != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("msb", ToString::to_string(&self.msb).as_str())?;
        }
        if self.lsb != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lsb", ToString::to_string(&self.lsb).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Uuid4 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msb",
            "lsb",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Msb,
            Lsb,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "msb" => Ok(GeneratedField::Msb),
                            "lsb" => Ok(GeneratedField::Lsb),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Uuid4;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.Uuid4")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Uuid4, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msb__ = None;
                let mut lsb__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Msb => {
                            if msb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msb"));
                            }
                            msb__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Lsb => {
                            if lsb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lsb"));
                            }
                            lsb__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Uuid4 {
                    msb: msb__.unwrap_or_default(),
                    lsb: lsb__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.Uuid4", FIELDS, GeneratedVisitor)
    }
}
