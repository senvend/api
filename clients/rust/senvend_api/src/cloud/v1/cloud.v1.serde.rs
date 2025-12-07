// @generated
impl serde::Serialize for CloudAgeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.target.is_some() {
            len += 1;
        }
        if self.age_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cloud.v1.CloudAgeRequest", len)?;
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        if let Some(v) = self.age_request.as_ref() {
            struct_ser.serialize_field("ageRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CloudAgeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
            "age_request",
            "ageRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
            AgeRequest,
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
                            "target" => Ok(GeneratedField::Target),
                            "ageRequest" | "age_request" => Ok(GeneratedField::AgeRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CloudAgeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cloud.v1.CloudAgeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CloudAgeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target__ = None;
                let mut age_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                        GeneratedField::AgeRequest => {
                            if age_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageRequest"));
                            }
                            age_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CloudAgeRequest {
                    target: target__,
                    age_request: age_request__,
                })
            }
        }
        deserializer.deserialize_struct("cloud.v1.CloudAgeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CloudAgeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.age_response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cloud.v1.CloudAgeResponse", len)?;
        if let Some(v) = self.age_response.as_ref() {
            struct_ser.serialize_field("ageResponse", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CloudAgeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "age_response",
            "ageResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgeResponse,
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
                            "ageResponse" | "age_response" => Ok(GeneratedField::AgeResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CloudAgeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cloud.v1.CloudAgeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CloudAgeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut age_response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgeResponse => {
                            if age_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageResponse"));
                            }
                            age_response__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CloudAgeResponse {
                    age_response: age_response__,
                })
            }
        }
        deserializer.deserialize_struct("cloud.v1.CloudAgeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CloudPayRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.target.is_some() {
            len += 1;
        }
        if self.pay_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cloud.v1.CloudPayRequest", len)?;
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        if let Some(v) = self.pay_request.as_ref() {
            struct_ser.serialize_field("payRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CloudPayRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
            "pay_request",
            "payRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
            PayRequest,
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
                            "target" => Ok(GeneratedField::Target),
                            "payRequest" | "pay_request" => Ok(GeneratedField::PayRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CloudPayRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cloud.v1.CloudPayRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CloudPayRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target__ = None;
                let mut pay_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                        GeneratedField::PayRequest => {
                            if pay_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payRequest"));
                            }
                            pay_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CloudPayRequest {
                    target: target__,
                    pay_request: pay_request__,
                })
            }
        }
        deserializer.deserialize_struct("cloud.v1.CloudPayRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CloudPayResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pay_response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cloud.v1.CloudPayResponse", len)?;
        if let Some(v) = self.pay_response.as_ref() {
            struct_ser.serialize_field("payResponse", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CloudPayResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pay_response",
            "payResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PayResponse,
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
                            "payResponse" | "pay_response" => Ok(GeneratedField::PayResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CloudPayResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cloud.v1.CloudPayResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CloudPayResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pay_response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PayResponse => {
                            if pay_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payResponse"));
                            }
                            pay_response__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CloudPayResponse {
                    pay_response: pay_response__,
                })
            }
        }
        deserializer.deserialize_struct("cloud.v1.CloudPayResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CloudTargetSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.selector.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cloud.v1.CloudTargetSelector", len)?;
        if let Some(v) = self.selector.as_ref() {
            match v {
                cloud_target_selector::Selector::MachineId(v) => {
                    struct_ser.serialize_field("machineId", v)?;
                }
                cloud_target_selector::Selector::SerialNumber(v) => {
                    struct_ser.serialize_field("serialNumber", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CloudTargetSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "machine_id",
            "machineId",
            "serial_number",
            "serialNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MachineId,
            SerialNumber,
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
                            "machineId" | "machine_id" => Ok(GeneratedField::MachineId),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CloudTargetSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cloud.v1.CloudTargetSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CloudTargetSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selector__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MachineId => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machineId"));
                            }
                            selector__ = map_.next_value::<::std::option::Option<_>>()?.map(cloud_target_selector::Selector::MachineId)
;
                        }
                        GeneratedField::SerialNumber => {
                            if selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            selector__ = map_.next_value::<::std::option::Option<_>>()?.map(cloud_target_selector::Selector::SerialNumber);
                        }
                    }
                }
                Ok(CloudTargetSelector {
                    selector: selector__,
                })
            }
        }
        deserializer.deserialize_struct("cloud.v1.CloudTargetSelector", FIELDS, GeneratedVisitor)
    }
}
