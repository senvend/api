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
            Self::CancelFailed => "AGE_API_FAILURE_REASON_CANCEL_FAILED",
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
            "AGE_API_FAILURE_REASON_CANCEL_FAILED",
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
                    "AGE_API_FAILURE_REASON_CANCEL_FAILED" => Ok(AgeApiFailureReason::CancelFailed),
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
            Self::ApproveAccepted => "AGE_API_SUCCESS_REASON_APPROVE_ACCEPTED",
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
            "AGE_API_SUCCESS_REASON_APPROVE_ACCEPTED",
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
                    "AGE_API_SUCCESS_REASON_APPROVE_ACCEPTED" => Ok(AgeApiSuccessReason::ApproveAccepted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgeApproveRequest {
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
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeApproveRequest", len)?;
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgeApproveRequest {
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
            type Value = AgeApproveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AgeApproveRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgeApproveRequest, V::Error>
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
                            reason__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgeApproveRequest {
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AgeApproveRequest", FIELDS, GeneratedVisitor)
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
            Self::InvalidUuid => "AGE_FAILURE_REASON_INVALID_UUID",
            Self::NoApplicableProvider => "AGE_FAILURE_REASON_NO_APPLICABLE_PROVIDER",
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
            "AGE_FAILURE_REASON_INVALID_UUID",
            "AGE_FAILURE_REASON_NO_APPLICABLE_PROVIDER",
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
                    "AGE_FAILURE_REASON_INVALID_UUID" => Ok(AgeFailureReason::InvalidUuid),
                    "AGE_FAILURE_REASON_NO_APPLICABLE_PROVIDER" => Ok(AgeFailureReason::NoApplicableProvider),
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
        if self.determined_age.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeFailureUnderage", len)?;
        if let Some(v) = self.determined_age.as_ref() {
            struct_ser.serialize_field("determinedAge", v)?;
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
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(AgeFailureUnderage {
                    determined_age: determined_age__,
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
                age_request::Request::Approve(v) => {
                    struct_ser.serialize_field("approve", v)?;
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
            "approve",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Start,
            Cancel,
            Approve,
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
                            "approve" => Ok(GeneratedField::Approve),
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
                        GeneratedField::Approve => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("approve"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(age_request::Request::Approve)
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
        if self.auto_cancel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AgeStartRequest", len)?;
        if self.min_age != 0 {
            struct_ser.serialize_field("minAge", &self.min_age)?;
        }
        if let Some(v) = self.auto_cancel.as_ref() {
            struct_ser.serialize_field("autoCancel", v)?;
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
            "auto_cancel",
            "autoCancel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinAge,
            AutoCancel,
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
                            "autoCancel" | "auto_cancel" => Ok(GeneratedField::AutoCancel),
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
                let mut auto_cancel__ = None;
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
                        GeneratedField::AutoCancel => {
                            if auto_cancel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoCancel"));
                            }
                            auto_cancel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgeStartRequest {
                    min_age: min_age__.unwrap_or_default(),
                    auto_cancel: auto_cancel__,
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
impl serde::Serialize for LineItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price.is_some() {
            len += 1;
        }
        if self.quantity != 0 {
            len += 1;
        }
        if self.product.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.LineItem", len)?;
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        if self.quantity != 0 {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if let Some(v) = self.product.as_ref() {
            match v {
                line_item::Product::Selection(v) => {
                    struct_ser.serialize_field("selection", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LineItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price",
            "quantity",
            "selection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Price,
            Quantity,
            Selection,
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
                            "price" => Ok(GeneratedField::Price),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "selection" => Ok(GeneratedField::Selection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LineItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.LineItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LineItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price__ = None;
                let mut quantity__ = None;
                let mut product__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Selection => {
                            if product__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selection"));
                            }
                            product__ = map_.next_value::<::std::option::Option<_>>()?.map(line_item::Product::Selection)
;
                        }
                    }
                }
                Ok(LineItem {
                    price: price__,
                    quantity: quantity__.unwrap_or_default(),
                    product: product__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.LineItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MdbSelectionDeniedReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MDB_SELECTION_DENIED_REASON_UNSPECIFIED",
            Self::UnknownErrorOutOfOrder => "MDB_SELECTION_DENIED_REASON_UNKNOWN_ERROR_OUT_OF_ORDER",
            Self::SelectionDoesNotExistNotConfigured => "MDB_SELECTION_DENIED_REASON_SELECTION_DOES_NOT_EXIST_NOT_CONFIGURED",
            Self::SelectionEmptyNotDispensed => "MDB_SELECTION_DENIED_REASON_SELECTION_EMPTY_NOT_DISPENSED",
            Self::SelectionDefective => "MDB_SELECTION_DENIED_REASON_SELECTION_DEFECTIVE",
            Self::IngredientIsOver => "MDB_SELECTION_DENIED_REASON_INGREDIENT_IS_OVER",
            Self::SelectionBlocked => "MDB_SELECTION_DENIED_REASON_SELECTION_BLOCKED",
            Self::SelectionInhibited => "MDB_SELECTION_DENIED_REASON_SELECTION_INHIBITED",
            Self::ProductExpired => "MDB_SELECTION_DENIED_REASON_PRODUCT_EXPIRED",
            Self::TemperatureConditionsOutOfRange => "MDB_SELECTION_DENIED_REASON_TEMPERATURE_CONDITIONS_OUT_OF_RANGE",
            Self::MachineBusyOrWrongMode => "MDB_SELECTION_DENIED_REASON_MACHINE_BUSY_OR_WRONG_MODE",
            Self::InsufficientCredit => "MDB_SELECTION_DENIED_REASON_INSUFFICIENT_CREDIT",
            Self::MachineTemporaryBusy => "MDB_SELECTION_DENIED_REASON_MACHINE_TEMPORARY_BUSY",
            Self::MaxRetryExceeded => "MDB_SELECTION_DENIED_REASON_MAX_RETRY_EXCEEDED",
            Self::UnknownError => "MDB_SELECTION_DENIED_REASON_UNKNOWN_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MdbSelectionDeniedReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MDB_SELECTION_DENIED_REASON_UNSPECIFIED",
            "MDB_SELECTION_DENIED_REASON_UNKNOWN_ERROR_OUT_OF_ORDER",
            "MDB_SELECTION_DENIED_REASON_SELECTION_DOES_NOT_EXIST_NOT_CONFIGURED",
            "MDB_SELECTION_DENIED_REASON_SELECTION_EMPTY_NOT_DISPENSED",
            "MDB_SELECTION_DENIED_REASON_SELECTION_DEFECTIVE",
            "MDB_SELECTION_DENIED_REASON_INGREDIENT_IS_OVER",
            "MDB_SELECTION_DENIED_REASON_SELECTION_BLOCKED",
            "MDB_SELECTION_DENIED_REASON_SELECTION_INHIBITED",
            "MDB_SELECTION_DENIED_REASON_PRODUCT_EXPIRED",
            "MDB_SELECTION_DENIED_REASON_TEMPERATURE_CONDITIONS_OUT_OF_RANGE",
            "MDB_SELECTION_DENIED_REASON_MACHINE_BUSY_OR_WRONG_MODE",
            "MDB_SELECTION_DENIED_REASON_INSUFFICIENT_CREDIT",
            "MDB_SELECTION_DENIED_REASON_MACHINE_TEMPORARY_BUSY",
            "MDB_SELECTION_DENIED_REASON_MAX_RETRY_EXCEEDED",
            "MDB_SELECTION_DENIED_REASON_UNKNOWN_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MdbSelectionDeniedReason;

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
                    "MDB_SELECTION_DENIED_REASON_UNSPECIFIED" => Ok(MdbSelectionDeniedReason::Unspecified),
                    "MDB_SELECTION_DENIED_REASON_UNKNOWN_ERROR_OUT_OF_ORDER" => Ok(MdbSelectionDeniedReason::UnknownErrorOutOfOrder),
                    "MDB_SELECTION_DENIED_REASON_SELECTION_DOES_NOT_EXIST_NOT_CONFIGURED" => Ok(MdbSelectionDeniedReason::SelectionDoesNotExistNotConfigured),
                    "MDB_SELECTION_DENIED_REASON_SELECTION_EMPTY_NOT_DISPENSED" => Ok(MdbSelectionDeniedReason::SelectionEmptyNotDispensed),
                    "MDB_SELECTION_DENIED_REASON_SELECTION_DEFECTIVE" => Ok(MdbSelectionDeniedReason::SelectionDefective),
                    "MDB_SELECTION_DENIED_REASON_INGREDIENT_IS_OVER" => Ok(MdbSelectionDeniedReason::IngredientIsOver),
                    "MDB_SELECTION_DENIED_REASON_SELECTION_BLOCKED" => Ok(MdbSelectionDeniedReason::SelectionBlocked),
                    "MDB_SELECTION_DENIED_REASON_SELECTION_INHIBITED" => Ok(MdbSelectionDeniedReason::SelectionInhibited),
                    "MDB_SELECTION_DENIED_REASON_PRODUCT_EXPIRED" => Ok(MdbSelectionDeniedReason::ProductExpired),
                    "MDB_SELECTION_DENIED_REASON_TEMPERATURE_CONDITIONS_OUT_OF_RANGE" => Ok(MdbSelectionDeniedReason::TemperatureConditionsOutOfRange),
                    "MDB_SELECTION_DENIED_REASON_MACHINE_BUSY_OR_WRONG_MODE" => Ok(MdbSelectionDeniedReason::MachineBusyOrWrongMode),
                    "MDB_SELECTION_DENIED_REASON_INSUFFICIENT_CREDIT" => Ok(MdbSelectionDeniedReason::InsufficientCredit),
                    "MDB_SELECTION_DENIED_REASON_MACHINE_TEMPORARY_BUSY" => Ok(MdbSelectionDeniedReason::MachineTemporaryBusy),
                    "MDB_SELECTION_DENIED_REASON_MAX_RETRY_EXCEEDED" => Ok(MdbSelectionDeniedReason::MaxRetryExceeded),
                    "MDB_SELECTION_DENIED_REASON_UNKNOWN_ERROR" => Ok(MdbSelectionDeniedReason::UnknownError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MdbVendFailureReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MDB_VEND_FAILURE_REASON_UNSPECIFIED",
            Self::UnknownOutOfOrder => "MDB_VEND_FAILURE_REASON_UNKNOWN_OUT_OF_ORDER",
            Self::SelectionDoesNotExistNotConfigured => "MDB_VEND_FAILURE_REASON_SELECTION_DOES_NOT_EXIST_NOT_CONFIGURED",
            Self::SelectionEmptyProductNotDispensed => "MDB_VEND_FAILURE_REASON_SELECTION_EMPTY_PRODUCT_NOT_DISPENSED",
            Self::SelectionDefective => "MDB_VEND_FAILURE_REASON_SELECTION_DEFECTIVE",
            Self::IngredientIsOver => "MDB_VEND_FAILURE_REASON_INGREDIENT_IS_OVER",
            Self::SelectionBlocked => "MDB_VEND_FAILURE_REASON_SELECTION_BLOCKED",
            Self::SelectionInhibited => "MDB_VEND_FAILURE_REASON_SELECTION_INHIBITED",
            Self::ProductExpired => "MDB_VEND_FAILURE_REASON_PRODUCT_EXPIRED",
            Self::TemperatureConditionsOutOfRange => "MDB_VEND_FAILURE_REASON_TEMPERATURE_CONDITIONS_OUT_OF_RANGE",
            Self::MachineBusyWrongMode => "MDB_VEND_FAILURE_REASON_MACHINE_BUSY_WRONG_MODE",
            Self::InsufficientCredit => "MDB_VEND_FAILURE_REASON_INSUFFICIENT_CREDIT",
            Self::Unknown => "MDB_VEND_FAILURE_REASON_UNKNOWN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MdbVendFailureReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MDB_VEND_FAILURE_REASON_UNSPECIFIED",
            "MDB_VEND_FAILURE_REASON_UNKNOWN_OUT_OF_ORDER",
            "MDB_VEND_FAILURE_REASON_SELECTION_DOES_NOT_EXIST_NOT_CONFIGURED",
            "MDB_VEND_FAILURE_REASON_SELECTION_EMPTY_PRODUCT_NOT_DISPENSED",
            "MDB_VEND_FAILURE_REASON_SELECTION_DEFECTIVE",
            "MDB_VEND_FAILURE_REASON_INGREDIENT_IS_OVER",
            "MDB_VEND_FAILURE_REASON_SELECTION_BLOCKED",
            "MDB_VEND_FAILURE_REASON_SELECTION_INHIBITED",
            "MDB_VEND_FAILURE_REASON_PRODUCT_EXPIRED",
            "MDB_VEND_FAILURE_REASON_TEMPERATURE_CONDITIONS_OUT_OF_RANGE",
            "MDB_VEND_FAILURE_REASON_MACHINE_BUSY_WRONG_MODE",
            "MDB_VEND_FAILURE_REASON_INSUFFICIENT_CREDIT",
            "MDB_VEND_FAILURE_REASON_UNKNOWN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MdbVendFailureReason;

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
                    "MDB_VEND_FAILURE_REASON_UNSPECIFIED" => Ok(MdbVendFailureReason::Unspecified),
                    "MDB_VEND_FAILURE_REASON_UNKNOWN_OUT_OF_ORDER" => Ok(MdbVendFailureReason::UnknownOutOfOrder),
                    "MDB_VEND_FAILURE_REASON_SELECTION_DOES_NOT_EXIST_NOT_CONFIGURED" => Ok(MdbVendFailureReason::SelectionDoesNotExistNotConfigured),
                    "MDB_VEND_FAILURE_REASON_SELECTION_EMPTY_PRODUCT_NOT_DISPENSED" => Ok(MdbVendFailureReason::SelectionEmptyProductNotDispensed),
                    "MDB_VEND_FAILURE_REASON_SELECTION_DEFECTIVE" => Ok(MdbVendFailureReason::SelectionDefective),
                    "MDB_VEND_FAILURE_REASON_INGREDIENT_IS_OVER" => Ok(MdbVendFailureReason::IngredientIsOver),
                    "MDB_VEND_FAILURE_REASON_SELECTION_BLOCKED" => Ok(MdbVendFailureReason::SelectionBlocked),
                    "MDB_VEND_FAILURE_REASON_SELECTION_INHIBITED" => Ok(MdbVendFailureReason::SelectionInhibited),
                    "MDB_VEND_FAILURE_REASON_PRODUCT_EXPIRED" => Ok(MdbVendFailureReason::ProductExpired),
                    "MDB_VEND_FAILURE_REASON_TEMPERATURE_CONDITIONS_OUT_OF_RANGE" => Ok(MdbVendFailureReason::TemperatureConditionsOutOfRange),
                    "MDB_VEND_FAILURE_REASON_MACHINE_BUSY_WRONG_MODE" => Ok(MdbVendFailureReason::MachineBusyWrongMode),
                    "MDB_VEND_FAILURE_REASON_INSUFFICIENT_CREDIT" => Ok(MdbVendFailureReason::InsufficientCredit),
                    "MDB_VEND_FAILURE_REASON_UNKNOWN" => Ok(MdbVendFailureReason::Unknown),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
            Self::CancelFailed => "PAY_API_FAILURE_REASON_CANCEL_FAILED",
            Self::UuidNotFound => "PAY_API_FAILURE_REASON_UUID_NOT_FOUND",
            Self::UnknownCommand => "PAY_API_FAILURE_REASON_UNKNOWN_COMMAND",
            Self::NoApprovedPayment => "PAY_API_FAILURE_REASON_NO_APPROVED_PAYMENT",
            Self::AmountMismatch => "PAY_API_FAILURE_REASON_AMOUNT_MISMATCH",
            Self::InvalidUuid => "PAY_API_FAILURE_REASON_INVALID_UUID",
            Self::InvalidState => "PAY_API_FAILURE_REASON_INVALID_STATE",
            Self::AlreadyApproved => "PAY_API_FAILURE_REASON_ALREADY_APPROVED",
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
            "PAY_API_FAILURE_REASON_CANCEL_FAILED",
            "PAY_API_FAILURE_REASON_UUID_NOT_FOUND",
            "PAY_API_FAILURE_REASON_UNKNOWN_COMMAND",
            "PAY_API_FAILURE_REASON_NO_APPROVED_PAYMENT",
            "PAY_API_FAILURE_REASON_AMOUNT_MISMATCH",
            "PAY_API_FAILURE_REASON_INVALID_UUID",
            "PAY_API_FAILURE_REASON_INVALID_STATE",
            "PAY_API_FAILURE_REASON_ALREADY_APPROVED",
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
                    "PAY_API_FAILURE_REASON_CANCEL_FAILED" => Ok(PayApiFailureReason::CancelFailed),
                    "PAY_API_FAILURE_REASON_UUID_NOT_FOUND" => Ok(PayApiFailureReason::UuidNotFound),
                    "PAY_API_FAILURE_REASON_UNKNOWN_COMMAND" => Ok(PayApiFailureReason::UnknownCommand),
                    "PAY_API_FAILURE_REASON_NO_APPROVED_PAYMENT" => Ok(PayApiFailureReason::NoApprovedPayment),
                    "PAY_API_FAILURE_REASON_AMOUNT_MISMATCH" => Ok(PayApiFailureReason::AmountMismatch),
                    "PAY_API_FAILURE_REASON_INVALID_UUID" => Ok(PayApiFailureReason::InvalidUuid),
                    "PAY_API_FAILURE_REASON_INVALID_STATE" => Ok(PayApiFailureReason::InvalidState),
                    "PAY_API_FAILURE_REASON_ALREADY_APPROVED" => Ok(PayApiFailureReason::AlreadyApproved),
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
            Self::AgeApproveAccepted => "PAY_API_SUCCESS_REASON_AGE_APPROVE_ACCEPTED",
            Self::UpdateAccepted => "PAY_API_SUCCESS_REASON_UPDATE_ACCEPTED",
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
            "PAY_API_SUCCESS_REASON_AGE_APPROVE_ACCEPTED",
            "PAY_API_SUCCESS_REASON_UPDATE_ACCEPTED",
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
                    "PAY_API_SUCCESS_REASON_AGE_APPROVE_ACCEPTED" => Ok(PayApiSuccessReason::AgeApproveAccepted),
                    "PAY_API_SUCCESS_REASON_UPDATE_ACCEPTED" => Ok(PayApiSuccessReason::UpdateAccepted),
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
        let mut len = 0;
        if self.amount != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayApproved", len)?;
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
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
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
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
                let mut amount__ = None;
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
                    }
                }
                Ok(PayApproved {
                    amount: amount__.unwrap_or_default(),
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
            Self::CompletionFailed => "PAY_FAILURE_REASON_COMPLETION_FAILED",
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
            "PAY_FAILURE_REASON_COMPLETION_FAILED",
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
                    "PAY_FAILURE_REASON_COMPLETION_FAILED" => Ok(PayFailureReason::CompletionFailed),
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
        if !self.line_items.is_empty() {
            len += 1;
        }
        if self.cash_amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayGoodsIssued", len)?;
        if self.partial_amount != 0 {
            struct_ser.serialize_field("partialAmount", &self.partial_amount)?;
        }
        if !self.line_items.is_empty() {
            struct_ser.serialize_field("lineItems", &self.line_items)?;
        }
        if let Some(v) = self.cash_amount.as_ref() {
            struct_ser.serialize_field("cashAmount", v)?;
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
            "line_items",
            "lineItems",
            "cash_amount",
            "cashAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartialAmount,
            LineItems,
            CashAmount,
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
                            "lineItems" | "line_items" => Ok(GeneratedField::LineItems),
                            "cashAmount" | "cash_amount" => Ok(GeneratedField::CashAmount),
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
                let mut line_items__ = None;
                let mut cash_amount__ = None;
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
                        GeneratedField::LineItems => {
                            if line_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineItems"));
                            }
                            line_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CashAmount => {
                            if cash_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cashAmount"));
                            }
                            cash_amount__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PayGoodsIssued {
                    partial_amount: partial_amount__.unwrap_or_default(),
                    line_items: line_items__.unwrap_or_default(),
                    cash_amount: cash_amount__,
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
                pay_request::Request::AgeApprove(v) => {
                    struct_ser.serialize_field("ageApprove", v)?;
                }
                pay_request::Request::Update(v) => {
                    struct_ser.serialize_field("update", v)?;
                }
                pay_request::Request::Vend(v) => {
                    struct_ser.serialize_field("vend", v)?;
                }
                pay_request::Request::VendCancel(v) => {
                    struct_ser.serialize_field("vendCancel", v)?;
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
            "age_approve",
            "ageApprove",
            "update",
            "vend",
            "vend_cancel",
            "vendCancel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Start,
            Cancel,
            GoodsIssued,
            AgeApprove,
            Update,
            Vend,
            VendCancel,
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
                            "ageApprove" | "age_approve" => Ok(GeneratedField::AgeApprove),
                            "update" => Ok(GeneratedField::Update),
                            "vend" => Ok(GeneratedField::Vend),
                            "vendCancel" | "vend_cancel" => Ok(GeneratedField::VendCancel),
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
                        GeneratedField::AgeApprove => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageApprove"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_request::Request::AgeApprove)
;
                        }
                        GeneratedField::Update => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_request::Request::Update)
;
                        }
                        GeneratedField::Vend => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vend"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_request::Request::Vend)
;
                        }
                        GeneratedField::VendCancel => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendCancel"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_request::Request::VendCancel)
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
                pay_response::Result::VendEvent(v) => {
                    struct_ser.serialize_field("vendEvent", v)?;
                }
                pay_response::Result::VendApiSuccess(v) => {
                    struct_ser.serialize_field("vendApiSuccess", v)?;
                }
                pay_response::Result::VendApiFailure(v) => {
                    struct_ser.serialize_field("vendApiFailure", v)?;
                }
                pay_response::Result::VendSuccess(v) => {
                    struct_ser.serialize_field("vendSuccess", v)?;
                }
                pay_response::Result::VendFailure(v) => {
                    struct_ser.serialize_field("vendFailure", v)?;
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
            "vend_event",
            "vendEvent",
            "vend_api_success",
            "vendApiSuccess",
            "vend_api_failure",
            "vendApiFailure",
            "vend_success",
            "vendSuccess",
            "vend_failure",
            "vendFailure",
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
            VendEvent,
            VendApiSuccess,
            VendApiFailure,
            VendSuccess,
            VendFailure,
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
                            "vendEvent" | "vend_event" => Ok(GeneratedField::VendEvent),
                            "vendApiSuccess" | "vend_api_success" => Ok(GeneratedField::VendApiSuccess),
                            "vendApiFailure" | "vend_api_failure" => Ok(GeneratedField::VendApiFailure),
                            "vendSuccess" | "vend_success" => Ok(GeneratedField::VendSuccess),
                            "vendFailure" | "vend_failure" => Ok(GeneratedField::VendFailure),
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
                        GeneratedField::VendEvent => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendEvent"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::VendEvent)
;
                        }
                        GeneratedField::VendApiSuccess => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendApiSuccess"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::VendApiSuccess)
;
                        }
                        GeneratedField::VendApiFailure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendApiFailure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::VendApiFailure)
;
                        }
                        GeneratedField::VendSuccess => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendSuccess"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::VendSuccess)
;
                        }
                        GeneratedField::VendFailure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendFailure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(pay_response::Result::VendFailure)
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
        if !self.line_items.is_empty() {
            len += 1;
        }
        if self.cash_amount.is_some() {
            len += 1;
        }
        if self.auto_cancel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayStart", len)?;
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if let Some(v) = self.age_verification.as_ref() {
            struct_ser.serialize_field("ageVerification", v)?;
        }
        if !self.line_items.is_empty() {
            struct_ser.serialize_field("lineItems", &self.line_items)?;
        }
        if let Some(v) = self.cash_amount.as_ref() {
            struct_ser.serialize_field("cashAmount", v)?;
        }
        if let Some(v) = self.auto_cancel.as_ref() {
            struct_ser.serialize_field("autoCancel", v)?;
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
            "line_items",
            "lineItems",
            "cash_amount",
            "cashAmount",
            "auto_cancel",
            "autoCancel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            AgeVerification,
            LineItems,
            CashAmount,
            AutoCancel,
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
                            "lineItems" | "line_items" => Ok(GeneratedField::LineItems),
                            "cashAmount" | "cash_amount" => Ok(GeneratedField::CashAmount),
                            "autoCancel" | "auto_cancel" => Ok(GeneratedField::AutoCancel),
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
                let mut line_items__ = None;
                let mut cash_amount__ = None;
                let mut auto_cancel__ = None;
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
                        GeneratedField::LineItems => {
                            if line_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineItems"));
                            }
                            line_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CashAmount => {
                            if cash_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cashAmount"));
                            }
                            cash_amount__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutoCancel => {
                            if auto_cancel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoCancel"));
                            }
                            auto_cancel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PayStart {
                    amount: amount__.unwrap_or_default(),
                    age_verification: age_verification__,
                    line_items: line_items__.unwrap_or_default(),
                    cash_amount: cash_amount__,
                    auto_cancel: auto_cancel__,
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
impl serde::Serialize for PayUpdate {
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
        if !self.line_items.is_empty() {
            len += 1;
        }
        if self.cash_amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PayUpdate", len)?;
        if self.partial_amount != 0 {
            struct_ser.serialize_field("partialAmount", &self.partial_amount)?;
        }
        if !self.line_items.is_empty() {
            struct_ser.serialize_field("lineItems", &self.line_items)?;
        }
        if let Some(v) = self.cash_amount.as_ref() {
            struct_ser.serialize_field("cashAmount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partial_amount",
            "partialAmount",
            "line_items",
            "lineItems",
            "cash_amount",
            "cashAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartialAmount,
            LineItems,
            CashAmount,
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
                            "lineItems" | "line_items" => Ok(GeneratedField::LineItems),
                            "cashAmount" | "cash_amount" => Ok(GeneratedField::CashAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PayUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PayUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PayUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partial_amount__ = None;
                let mut line_items__ = None;
                let mut cash_amount__ = None;
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
                        GeneratedField::LineItems => {
                            if line_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineItems"));
                            }
                            line_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CashAmount => {
                            if cash_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cashAmount"));
                            }
                            cash_amount__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PayUpdate {
                    partial_amount: partial_amount__.unwrap_or_default(),
                    line_items: line_items__.unwrap_or_default(),
                    cash_amount: cash_amount__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PayUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Selection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.slot != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.Selection", len)?;
        if self.slot != 0 {
            struct_ser.serialize_field("slot", &self.slot)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Selection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "slot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Slot,
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
                            "slot" => Ok(GeneratedField::Slot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Selection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.Selection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Selection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut slot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Slot => {
                            if slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slot"));
                            }
                            slot__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Selection {
                    slot: slot__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.Selection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SemanticVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.major != 0 {
            len += 1;
        }
        if self.minor != 0 {
            len += 1;
        }
        if self.patch != 0 {
            len += 1;
        }
        if self.branch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.SemanticVersion", len)?;
        if self.major != 0 {
            struct_ser.serialize_field("major", &self.major)?;
        }
        if self.minor != 0 {
            struct_ser.serialize_field("minor", &self.minor)?;
        }
        if self.patch != 0 {
            struct_ser.serialize_field("patch", &self.patch)?;
        }
        if let Some(v) = self.branch.as_ref() {
            struct_ser.serialize_field("branch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SemanticVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "major",
            "minor",
            "patch",
            "branch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Major,
            Minor,
            Patch,
            Branch,
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
                            "major" => Ok(GeneratedField::Major),
                            "minor" => Ok(GeneratedField::Minor),
                            "patch" => Ok(GeneratedField::Patch),
                            "branch" => Ok(GeneratedField::Branch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SemanticVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.SemanticVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SemanticVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut major__ = None;
                let mut minor__ = None;
                let mut patch__ = None;
                let mut branch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Major => {
                            if major__.is_some() {
                                return Err(serde::de::Error::duplicate_field("major"));
                            }
                            major__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Minor => {
                            if minor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minor"));
                            }
                            minor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Patch => {
                            if patch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("patch"));
                            }
                            patch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Branch => {
                            if branch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("branch"));
                            }
                            branch__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SemanticVersion {
                    major: major__.unwrap_or_default(),
                    minor: minor__.unwrap_or_default(),
                    patch: patch__.unwrap_or_default(),
                    branch: branch__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.SemanticVersion", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for VendApiFailure {
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
        let mut struct_ser = serializer.serialize_struct("api.v1.VendApiFailure", len)?;
        if self.reason != 0 {
            let v = VendApiFailureReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendApiFailure {
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
            type Value = VendApiFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendApiFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendApiFailure, V::Error>
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
                            reason__ = Some(map_.next_value::<VendApiFailureReason>()? as i32);
                        }
                    }
                }
                Ok(VendApiFailure {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendApiFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendApiFailureReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VEND_API_FAILURE_REASON_UNSPECIFIED",
            Self::InvalidQuantity => "VEND_API_FAILURE_REASON_INVALID_QUANTITY",
            Self::CancelFailed => "VEND_API_FAILURE_REASON_CANCEL_FAILED",
            Self::UuidNotFound => "VEND_API_FAILURE_REASON_UUID_NOT_FOUND",
            Self::UnknownCommand => "VEND_API_FAILURE_REASON_UNKNOWN_COMMAND",
            Self::InvalidUuid => "VEND_API_FAILURE_REASON_INVALID_UUID",
            Self::InvalidState => "VEND_API_FAILURE_REASON_INVALID_STATE",
            Self::VendingOngoing => "VEND_API_FAILURE_REASON_VENDING_ONGOING",
            Self::InvalidConfig => "VEND_API_FAILURE_REASON_INVALID_CONFIG",
            Self::MdbDisabled => "VEND_API_FAILURE_REASON_MDB_DISABLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VendApiFailureReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VEND_API_FAILURE_REASON_UNSPECIFIED",
            "VEND_API_FAILURE_REASON_INVALID_QUANTITY",
            "VEND_API_FAILURE_REASON_CANCEL_FAILED",
            "VEND_API_FAILURE_REASON_UUID_NOT_FOUND",
            "VEND_API_FAILURE_REASON_UNKNOWN_COMMAND",
            "VEND_API_FAILURE_REASON_INVALID_UUID",
            "VEND_API_FAILURE_REASON_INVALID_STATE",
            "VEND_API_FAILURE_REASON_VENDING_ONGOING",
            "VEND_API_FAILURE_REASON_INVALID_CONFIG",
            "VEND_API_FAILURE_REASON_MDB_DISABLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendApiFailureReason;

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
                    "VEND_API_FAILURE_REASON_UNSPECIFIED" => Ok(VendApiFailureReason::Unspecified),
                    "VEND_API_FAILURE_REASON_INVALID_QUANTITY" => Ok(VendApiFailureReason::InvalidQuantity),
                    "VEND_API_FAILURE_REASON_CANCEL_FAILED" => Ok(VendApiFailureReason::CancelFailed),
                    "VEND_API_FAILURE_REASON_UUID_NOT_FOUND" => Ok(VendApiFailureReason::UuidNotFound),
                    "VEND_API_FAILURE_REASON_UNKNOWN_COMMAND" => Ok(VendApiFailureReason::UnknownCommand),
                    "VEND_API_FAILURE_REASON_INVALID_UUID" => Ok(VendApiFailureReason::InvalidUuid),
                    "VEND_API_FAILURE_REASON_INVALID_STATE" => Ok(VendApiFailureReason::InvalidState),
                    "VEND_API_FAILURE_REASON_VENDING_ONGOING" => Ok(VendApiFailureReason::VendingOngoing),
                    "VEND_API_FAILURE_REASON_INVALID_CONFIG" => Ok(VendApiFailureReason::InvalidConfig),
                    "VEND_API_FAILURE_REASON_MDB_DISABLED" => Ok(VendApiFailureReason::MdbDisabled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for VendApiSuccess {
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
        let mut struct_ser = serializer.serialize_struct("api.v1.VendApiSuccess", len)?;
        if self.reason != 0 {
            let v = VendApiSuccessReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendApiSuccess {
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
            type Value = VendApiSuccess;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendApiSuccess")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendApiSuccess, V::Error>
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
                            reason__ = Some(map_.next_value::<VendApiSuccessReason>()? as i32);
                        }
                    }
                }
                Ok(VendApiSuccess {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendApiSuccess", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendApiSuccessReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VEND_API_SUCCESS_REASON_UNSPECIFIED",
            Self::VendingStarted => "VEND_API_SUCCESS_REASON_VENDING_STARTED",
            Self::CancelAccepted => "VEND_API_SUCCESS_REASON_CANCEL_ACCEPTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VendApiSuccessReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VEND_API_SUCCESS_REASON_UNSPECIFIED",
            "VEND_API_SUCCESS_REASON_VENDING_STARTED",
            "VEND_API_SUCCESS_REASON_CANCEL_ACCEPTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendApiSuccessReason;

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
                    "VEND_API_SUCCESS_REASON_UNSPECIFIED" => Ok(VendApiSuccessReason::Unspecified),
                    "VEND_API_SUCCESS_REASON_VENDING_STARTED" => Ok(VendApiSuccessReason::VendingStarted),
                    "VEND_API_SUCCESS_REASON_CANCEL_ACCEPTED" => Ok(VendApiSuccessReason::CancelAccepted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for VendCancel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.VendCancel", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendCancel {
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
            type Value = VendCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendCancel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(VendCancel {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendCancel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.line_item.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.VendEvent", len)?;
        if let Some(v) = self.line_item.as_ref() {
            struct_ser.serialize_field("lineItem", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            match v {
                vend_event::Result::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                vend_event::Result::Failure(v) => {
                    struct_ser.serialize_field("failure", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "line_item",
            "lineItem",
            "success",
            "failure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LineItem,
            Success,
            Failure,
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
                            "lineItem" | "line_item" => Ok(GeneratedField::LineItem),
                            "success" => Ok(GeneratedField::Success),
                            "failure" => Ok(GeneratedField::Failure),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut line_item__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LineItem => {
                            if line_item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineItem"));
                            }
                            line_item__ = map_.next_value()?;
                        }
                        GeneratedField::Success => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_event::Result::Success)
;
                        }
                        GeneratedField::Failure => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_event::Result::Failure)
;
                        }
                    }
                }
                Ok(VendEvent {
                    line_item: line_item__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendFailure {
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
        if self.successes != 0 {
            len += 1;
        }
        if self.protocol_error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.VendFailure", len)?;
        if self.reason != 0 {
            let v = VendFailureReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        if self.successes != 0 {
            struct_ser.serialize_field("successes", &self.successes)?;
        }
        if let Some(v) = self.protocol_error.as_ref() {
            match v {
                vend_failure::ProtocolError::MdbSelectionDenied(v) => {
                    let v = MdbSelectionDeniedReason::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("mdbSelectionDenied", &v)?;
                }
                vend_failure::ProtocolError::MdbVendFailure(v) => {
                    let v = MdbVendFailureReason::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("mdbVendFailure", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
            "successes",
            "mdb_selection_denied",
            "mdbSelectionDenied",
            "mdb_vend_failure",
            "mdbVendFailure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
            Successes,
            MdbSelectionDenied,
            MdbVendFailure,
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
                            "successes" => Ok(GeneratedField::Successes),
                            "mdbSelectionDenied" | "mdb_selection_denied" => Ok(GeneratedField::MdbSelectionDenied),
                            "mdbVendFailure" | "mdb_vend_failure" => Ok(GeneratedField::MdbVendFailure),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                let mut successes__ = None;
                let mut protocol_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<VendFailureReason>()? as i32);
                        }
                        GeneratedField::Successes => {
                            if successes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successes"));
                            }
                            successes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MdbSelectionDenied => {
                            if protocol_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mdbSelectionDenied"));
                            }
                            protocol_error__ = map_.next_value::<::std::option::Option<MdbSelectionDeniedReason>>()?.map(|x| vend_failure::ProtocolError::MdbSelectionDenied(x as i32));
                        }
                        GeneratedField::MdbVendFailure => {
                            if protocol_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mdbVendFailure"));
                            }
                            protocol_error__ = map_.next_value::<::std::option::Option<MdbVendFailureReason>>()?.map(|x| vend_failure::ProtocolError::MdbVendFailure(x as i32));
                        }
                    }
                }
                Ok(VendFailure {
                    reason: reason__.unwrap_or_default(),
                    successes: successes__.unwrap_or_default(),
                    protocol_error: protocol_error__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendFailureReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VEND_FAILURE_REASON_UNSPECIFIED",
            Self::Timeout => "VEND_FAILURE_REASON_TIMEOUT",
            Self::VendingFailed => "VEND_FAILURE_REASON_VENDING_FAILED",
            Self::ApiCancelled => "VEND_FAILURE_REASON_API_CANCELLED",
            Self::InvalidState => "VEND_FAILURE_REASON_INVALID_STATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VendFailureReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VEND_FAILURE_REASON_UNSPECIFIED",
            "VEND_FAILURE_REASON_TIMEOUT",
            "VEND_FAILURE_REASON_VENDING_FAILED",
            "VEND_FAILURE_REASON_API_CANCELLED",
            "VEND_FAILURE_REASON_INVALID_STATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendFailureReason;

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
                    "VEND_FAILURE_REASON_UNSPECIFIED" => Ok(VendFailureReason::Unspecified),
                    "VEND_FAILURE_REASON_TIMEOUT" => Ok(VendFailureReason::Timeout),
                    "VEND_FAILURE_REASON_VENDING_FAILED" => Ok(VendFailureReason::VendingFailed),
                    "VEND_FAILURE_REASON_API_CANCELLED" => Ok(VendFailureReason::ApiCancelled),
                    "VEND_FAILURE_REASON_INVALID_STATE" => Ok(VendFailureReason::InvalidState),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for VendRequest {
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
        let mut struct_ser = serializer.serialize_struct("api.v1.VendRequest", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                vend_request::Request::Start(v) => {
                    struct_ser.serialize_field("start", v)?;
                }
                vend_request::Request::Cancel(v) => {
                    struct_ser.serialize_field("cancel", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendRequest {
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
            type Value = VendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendRequest, V::Error>
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
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_request::Request::Start)
;
                        }
                        GeneratedField::Cancel => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancel"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_request::Request::Cancel)
;
                        }
                    }
                }
                Ok(VendRequest {
                    id: id__,
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendResponse {
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
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.VendResponse", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            match v {
                vend_response::Response::Event(v) => {
                    struct_ser.serialize_field("event", v)?;
                }
                vend_response::Response::ApiFailure(v) => {
                    struct_ser.serialize_field("apiFailure", v)?;
                }
                vend_response::Response::ApiSuccess(v) => {
                    struct_ser.serialize_field("apiSuccess", v)?;
                }
                vend_response::Response::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                vend_response::Response::Failure(v) => {
                    struct_ser.serialize_field("failure", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "event",
            "api_failure",
            "apiFailure",
            "api_success",
            "apiSuccess",
            "success",
            "failure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Event,
            ApiFailure,
            ApiSuccess,
            Success,
            Failure,
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
                            "event" => Ok(GeneratedField::Event),
                            "apiFailure" | "api_failure" => Ok(GeneratedField::ApiFailure),
                            "apiSuccess" | "api_success" => Ok(GeneratedField::ApiSuccess),
                            "success" => Ok(GeneratedField::Success),
                            "failure" => Ok(GeneratedField::Failure),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Event => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_response::Response::Event)
;
                        }
                        GeneratedField::ApiFailure => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiFailure"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_response::Response::ApiFailure)
;
                        }
                        GeneratedField::ApiSuccess => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiSuccess"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_response::Response::ApiSuccess)
;
                        }
                        GeneratedField::Success => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_response::Response::Success)
;
                        }
                        GeneratedField::Failure => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(vend_response::Response::Failure)
;
                        }
                    }
                }
                Ok(VendResponse {
                    id: id__,
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.VendResult", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
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
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VendResult {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendStart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.line_items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.VendStart", len)?;
        if !self.line_items.is_empty() {
            struct_ser.serialize_field("lineItems", &self.line_items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendStart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "line_items",
            "lineItems",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LineItems,
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
                            "lineItems" | "line_items" => Ok(GeneratedField::LineItems),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VendStart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendStart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendStart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut line_items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LineItems => {
                            if line_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lineItems"));
                            }
                            line_items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VendStart {
                    line_items: line_items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendStart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VendSuccess {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.VendSuccess", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VendSuccess {
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
            type Value = VendSuccess;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VendSuccess")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VendSuccess, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(VendSuccess {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VendSuccess", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VersionRequest {
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
        let mut struct_ser = serializer.serialize_struct("api.v1.VersionRequest", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VersionRequest {
                    id: id__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VersionResponse {
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
        if self.app_version.is_some() {
            len += 1;
        }
        if self.api_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.VersionResponse", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.app_version.as_ref() {
            struct_ser.serialize_field("appVersion", v)?;
        }
        if let Some(v) = self.api_version.as_ref() {
            struct_ser.serialize_field("apiVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "app_version",
            "appVersion",
            "api_version",
            "apiVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AppVersion,
            ApiVersion,
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
                            "appVersion" | "app_version" => Ok(GeneratedField::AppVersion),
                            "apiVersion" | "api_version" => Ok(GeneratedField::ApiVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.VersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut app_version__ = None;
                let mut api_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::AppVersion => {
                            if app_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appVersion"));
                            }
                            app_version__ = map_.next_value()?;
                        }
                        GeneratedField::ApiVersion => {
                            if api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiVersion"));
                            }
                            api_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VersionResponse {
                    id: id__,
                    app_version: app_version__,
                    api_version: api_version__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.VersionResponse", FIELDS, GeneratedVisitor)
    }
}
