// @generated
pub mod api {
    #[cfg(feature = "api-v1")]
    // @@protoc_insertion_point(attribute:api.v1)
    pub mod v1 {
        include!("api/v1/api.v1.rs");
        // @@protoc_insertion_point(api.v1)
    }
}
pub mod cloud {
    #[cfg(feature = "cloud-v1")]
    // @@protoc_insertion_point(attribute:cloud.v1)
    pub mod v1 {
        include!("cloud/v1/cloud.v1.rs");
        // @@protoc_insertion_point(cloud.v1)
    }
}
pub mod local {
    #[cfg(feature = "local-v1")]
    // @@protoc_insertion_point(attribute:local.v1)
    pub mod v1 {
        include!("local/v1/local.v1.rs");
        // @@protoc_insertion_point(local.v1)
    }
}
