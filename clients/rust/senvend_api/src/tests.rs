use uuid::Uuid;

use crate::protos::api::v1::Uuid4;

#[test]
fn uuid_serde() {
    let original_uuid = Uuid::new_v4();
    let proto_uuid = Uuid4::try_from(&original_uuid).unwrap();
    let converted_uuid = Uuid::try_from(&proto_uuid).unwrap();
    assert_eq!(original_uuid, converted_uuid);
    let proto_uuid_back = Uuid4::try_from(&converted_uuid).unwrap();
    assert_eq!(proto_uuid, proto_uuid_back);
}
