// @generated
/// Generated client implementations.
pub mod age_verification_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /**
This service provides the necessary functionality to handle age verification via the SENVEND Terminal.\
Only necessary if age verification is the sole purpose.\
The PayService contains its own way of starting age verification before the actual payment.
*/
    #[derive(Debug, Clone)]
    pub struct AgeVerificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AgeVerificationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AgeVerificationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AgeVerificationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AgeVerificationServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /**
Initiates an age verification process on the SENVEND terminal.\
Accepts a stream of AgeRequest for starting and controlling age verification.\
Returns a stream of AgeResponse containing status and error return messages.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given and a process is running, the request is applied to that running process.\
Otherwise a new UUID is generated per request.

- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.\
An empty id will work as well if the original request started the currently running process.

- All given ids must be valid version 4 UUIDs.

- If disconnected during a verification process, after a reconnect the currently running process can still be controlled.\
AgeResponses that occurred during the disconnect are lost though.

- If auto_cancel is left out or set to true, a new AgeRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.\
If given but false, sending a new AgeRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
</details>

<details open>
<summary>Process Constraints</summary>
- The minimum age to verify has to be greater than zero and can maximally be 120.\
Depending on the method chosen, only certain ages can be verified.

- Girocard: 16 or 18. Will be set to the next one above if below (e.g. to 18 if 17 is requested).\
Will not be available for selection if above 18 is requested. Will fail in older app versions (<1.7.0) instead.

- FaceScan: All ages supported. Might fail if actual age is very close to the requested one.

- Document scan: All ages supported.

- PayPal: 18 only. Will be set to 18 if below.\
Will not be available for selection if above 18 is requested. Will fail in older app versions (<1.7.0) instead.

- ID Austria: All ages supported.
</details>
*/
        pub async fn age(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::api::v1::AgeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::api::v1::AgeResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/local.v1.AgeVerificationService/Age",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("local.v1.AgeVerificationService", "Age"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod age_verification_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AgeVerificationServiceServer.
    #[async_trait]
    pub trait AgeVerificationService: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the Age method.
        type AgeStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::api::v1::AgeResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        /**
Initiates an age verification process on the SENVEND terminal.\
Accepts a stream of AgeRequest for starting and controlling age verification.\
Returns a stream of AgeResponse containing status and error return messages.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given and a process is running, the request is applied to that running process.\
Otherwise a new UUID is generated per request.

- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.\
An empty id will work as well if the original request started the currently running process.

- All given ids must be valid version 4 UUIDs.

- If disconnected during a verification process, after a reconnect the currently running process can still be controlled.\
AgeResponses that occurred during the disconnect are lost though.

- If auto_cancel is left out or set to true, a new AgeRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.\
If given but false, sending a new AgeRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
</details>

<details open>
<summary>Process Constraints</summary>
- The minimum age to verify has to be greater than zero and can maximally be 120.\
Depending on the method chosen, only certain ages can be verified.

- Girocard: 16 or 18. Will be set to the next one above if below (e.g. to 18 if 17 is requested).\
Will not be available for selection if above 18 is requested. Will fail in older app versions (<1.7.0) instead.

- FaceScan: All ages supported. Might fail if actual age is very close to the requested one.

- Document scan: All ages supported.

- PayPal: 18 only. Will be set to 18 if below.\
Will not be available for selection if above 18 is requested. Will fail in older app versions (<1.7.0) instead.

- ID Austria: All ages supported.
</details>
*/
        async fn age(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::api::v1::AgeRequest>,
            >,
        ) -> std::result::Result<tonic::Response<Self::AgeStream>, tonic::Status>;
    }
    /**
This service provides the necessary functionality to handle age verification via the SENVEND Terminal.\
Only necessary if age verification is the sole purpose.\
The PayService contains its own way of starting age verification before the actual payment.
*/
    #[derive(Debug)]
    pub struct AgeVerificationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AgeVerificationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for AgeVerificationServiceServer<T>
    where
        T: AgeVerificationService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/local.v1.AgeVerificationService/Age" => {
                    #[allow(non_camel_case_types)]
                    struct AgeSvc<T: AgeVerificationService>(pub Arc<T>);
                    impl<
                        T: AgeVerificationService,
                    > tonic::server::StreamingService<
                        super::super::super::api::v1::AgeRequest,
                    > for AgeSvc<T> {
                        type Response = super::super::super::api::v1::AgeResponse;
                        type ResponseStream = T::AgeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::api::v1::AgeRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AgeVerificationService>::age(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AgeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AgeVerificationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "local.v1.AgeVerificationService";
    impl<T> tonic::server::NamedService for AgeVerificationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod pay_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /**
This service provides the necessary functionality to handle payments via the SENVEND Terminal.\
Optionally, age verification can be enforced before the payment via the `PayStart` message.\
Optionally, vending is possible after APPROVE is received, either via this or via the `Vend` service.
*/
    #[derive(Debug, Clone)]
    pub struct PayServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PayServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PayServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PayServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            PayServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /**
Initiates a payment process on the SENVEND terminal.\
Accepts a stream of PayRequest for starting and controlling payments.\
Returns a stream of PayResponse containing status and error return messages.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given and a process is running, the request is applied to that running process.\
Otherwise a new UUID is generated per request.

- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.\
An empty id will work as well if the original request started the currently running process.

- All given ids must be valid version 4 UUIDs.

- If disconnected during a payment process, after a reconnect the currently running process can still be controlled.\
PayResponses that occurred during the disconnect are lost though.

- If auto_cancel is left out or set to true, a new PayRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.\
If given but false, sending a new PayRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
</details>

<details open>
<summary>Process Constraints</summary>

- The amount to charge is given in cents and can even be zero.\
The last option is useful to combine vending or age verification with a `GoodsIssued` message,\
mostly for telemetry purposes.

- The minimum age to verify has to be greater than zero and can maximally be 120.\
Depending on the method chosen, only certain ages can be verified.\
See the `Age` service for more details.

- An additional external age verification step can be implemented by sending an `AgeApproveRequest` message.\
This will mark the age verification as approved and continue with payment.

- If a payment was approved, a `GoodsIssued` message must be sent in order to finalize it.\
*The client has 9m30s to answer to the approval, or the goods will be issued to the customer as an emergency measure.**

- Vending can also be done via this endpoint by sending a VendStart message.\
These are accepted either when no payment is running, or after the payment was APPROVED and before sending GOODS_ISSUED.\
See `Vend` service for details.
</details>

<details>
<summary>Telemetry / Invoice Line Items / Mixed Payments</summary>

It is possible to send a list of products, their prices and the quantity per product sold alongside the `pay_start` and `goods_issued` requests. See the documentation of the api.v1.LineItems message.

Mixed payments can be supported by sending an additional cash_amount via the `pay_start` or `goods_issued` message.
The amount in PayStart.amount or PayGoodsIssued.partial_amount only covers cashless transactions,
therefore cash_amount is independent of that and only for reporting purposes via telemetry.

The device will do a verification of the payment amount (including cash_amount if present) versus the sum of the provided LineItem list, and report an API_ERROR if these amounts mismatch.

These messages are processed by the SENVEND web portal and taken into consideration when generating sales reports.

If LineItems or cash_amount are sent alongside the `goods_issued` message, they take precedence over any values from the `pay_start` message, effectively overriding them.
</details>

<details>
<summary>State and state changes</summary>

| State | Request | Result |
|--------------------------|----------------------------|------------------------------|
| No payment running | PayStart | Payment start |
| | PayStart (with AgeRequest) | AgeVerification start |
| | PayCancel | ApiError |
| | PayGoodsIssued | ApiError |
| Age verification ongoing | PayStart | ApiError |
| | PayCancel | AgeVerification cancel |
| | PayGoodsIssued | ApiError |
| | AgeApproveRequest | Terminal Proceeds to payment |
| Payment process ongoing | PayStart | ApiError |
| | PayCancel | Payment cancel |
| | PayGoodsIssued | ApiError |
| | AgeApproveRequest | ApiError |
| Payment accepted | PayStart | ApiError |
| | PayCancel | Reimburse and cancel payment |
| | PayGoodsIssued | Finalize payment |
</details>

<details>
<summary>State and errors</summary>

| State | Event | Error |
|--------------------------|---------------------------|-----------------------------------|
| Age verification ongoing | Took too long (timeout) | AGE_FAILURE_REASON_USER_CANCELLED |
| | User actively canceled | AGE_FAILURE_REASON_USER_CANCELLED |
| | Cancel via API | AGE_FAILURE_REASON_API_CANCELLED |
| | Verification fails | AgeFailureUnderage |
| Payment process ongoing | Took too long (timeout) | PAY_FAILURE_REASON_USER_CANCELLED |
| | User actively canceled | PAY_FAILURE_REASON_USER_CANCELLED |
| | Canceled via API | PAY_FAILURE_REASON_API_CANCELLED |
| | Payment failed (no debit) | PAY_FAILURE_REASON_PAYMENT_FAILED |
| Payment accepted | No response from API | PaySuccess |
</details>

<details>
<summary>Examples</summary>

###### Full payment process with age verification (no errors)

> **->** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}

- *Age Verification on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {}}

> **->** {"goods_issued":{}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}

###### Full payment process with external age verification (AgeApproveRequest)

> **->** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}

> **->** {"ageApprove": {}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {}}

> **->** {"goods_issued":{}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}

###### Payment process without age verification (no errors, using pre-generated UUIDs)

> **->** {"id":{"msb":5, "lsb":0}, "start": {"amount": 100}}

> **\<-** {"id": {"msb": "5"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "5"}, "approved": {}}

> **->** {"id": {"msb":5, "lsb":0}, "goods_issued": {}}

> **\<-** {"id": {"msb": "5"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED" }}

> **\<-** {"id": {"msb": "5"}, "success": {}}

###### Payment process without age verification (payment failed)

> **->** {"start": {"amount": 220}}

> **\<-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (failed)

> **\<-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "failure": {"failureReason": "PAY_FAILURE_REASON_PAYMENT_FAILED"}}

###### Full payment process with age verification (age verification failed)

> **->** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageApiSuccess": {"reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED"}}

- *Age Verification on device* (failed)

> **\<-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageFailure": {"underAge": {}}}

###### Payment process with detailed invoice tracking (i.e. basket)

> **->** {"start": {"amount": 100}}

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED" }}

- *payment on device* (success)

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 0, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "success": {}}

###### Payment process with invoice tracking change

> **->** {"start": {"amount": 100, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 60, "line_items": [{"price": 60, "quantity":1, "selection": {"slot":2}}] }}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

###### Mixed payment (cash paid before PaymentStart)
> **->** {"start": {"amount": 100, "cash_amount": 50}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {}}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

###### Mixed payment (cash amount changed before GoodsIssued)
> **->** {"start": {"amount": 100, "cash_amount": 50}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

###### Mixed payment (cash amount only known on GoodsIssued)
> **->** {"start": {"amount": 150}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

</details>
*/
        pub async fn pay(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::api::v1::PayRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::api::v1::PayResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/local.v1.PayService/Pay");
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new("local.v1.PayService", "Pay"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod pay_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PayServiceServer.
    #[async_trait]
    pub trait PayService: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the Pay method.
        type PayStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::api::v1::PayResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        /**
Initiates a payment process on the SENVEND terminal.\
Accepts a stream of PayRequest for starting and controlling payments.\
Returns a stream of PayResponse containing status and error return messages.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given and a process is running, the request is applied to that running process.\
Otherwise a new UUID is generated per request.

- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.\
An empty id will work as well if the original request started the currently running process.

- All given ids must be valid version 4 UUIDs.

- If disconnected during a payment process, after a reconnect the currently running process can still be controlled.\
PayResponses that occurred during the disconnect are lost though.

- If auto_cancel is left out or set to true, a new PayRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.\
If given but false, sending a new PayRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
</details>

<details open>
<summary>Process Constraints</summary>

- The amount to charge is given in cents and can even be zero.\
The last option is useful to combine vending or age verification with a `GoodsIssued` message,\
mostly for telemetry purposes.

- The minimum age to verify has to be greater than zero and can maximally be 120.\
Depending on the method chosen, only certain ages can be verified.\
See the `Age` service for more details.

- An additional external age verification step can be implemented by sending an `AgeApproveRequest` message.\
This will mark the age verification as approved and continue with payment.

- If a payment was approved, a `GoodsIssued` message must be sent in order to finalize it.\
*The client has 9m30s to answer to the approval, or the goods will be issued to the customer as an emergency measure.**

- Vending can also be done via this endpoint by sending a VendStart message.\
These are accepted either when no payment is running, or after the payment was APPROVED and before sending GOODS_ISSUED.\
See `Vend` service for details.
</details>

<details>
<summary>Telemetry / Invoice Line Items / Mixed Payments</summary>

It is possible to send a list of products, their prices and the quantity per product sold alongside the `pay_start` and `goods_issued` requests. See the documentation of the api.v1.LineItems message.

Mixed payments can be supported by sending an additional cash_amount via the `pay_start` or `goods_issued` message.
The amount in PayStart.amount or PayGoodsIssued.partial_amount only covers cashless transactions,
therefore cash_amount is independent of that and only for reporting purposes via telemetry.

The device will do a verification of the payment amount (including cash_amount if present) versus the sum of the provided LineItem list, and report an API_ERROR if these amounts mismatch.

These messages are processed by the SENVEND web portal and taken into consideration when generating sales reports.

If LineItems or cash_amount are sent alongside the `goods_issued` message, they take precedence over any values from the `pay_start` message, effectively overriding them.
</details>

<details>
<summary>State and state changes</summary>

| State | Request | Result |
|--------------------------|----------------------------|------------------------------|
| No payment running | PayStart | Payment start |
| | PayStart (with AgeRequest) | AgeVerification start |
| | PayCancel | ApiError |
| | PayGoodsIssued | ApiError |
| Age verification ongoing | PayStart | ApiError |
| | PayCancel | AgeVerification cancel |
| | PayGoodsIssued | ApiError |
| | AgeApproveRequest | Terminal Proceeds to payment |
| Payment process ongoing | PayStart | ApiError |
| | PayCancel | Payment cancel |
| | PayGoodsIssued | ApiError |
| | AgeApproveRequest | ApiError |
| Payment accepted | PayStart | ApiError |
| | PayCancel | Reimburse and cancel payment |
| | PayGoodsIssued | Finalize payment |
</details>

<details>
<summary>State and errors</summary>

| State | Event | Error |
|--------------------------|---------------------------|-----------------------------------|
| Age verification ongoing | Took too long (timeout) | AGE_FAILURE_REASON_USER_CANCELLED |
| | User actively canceled | AGE_FAILURE_REASON_USER_CANCELLED |
| | Cancel via API | AGE_FAILURE_REASON_API_CANCELLED |
| | Verification fails | AgeFailureUnderage |
| Payment process ongoing | Took too long (timeout) | PAY_FAILURE_REASON_USER_CANCELLED |
| | User actively canceled | PAY_FAILURE_REASON_USER_CANCELLED |
| | Canceled via API | PAY_FAILURE_REASON_API_CANCELLED |
| | Payment failed (no debit) | PAY_FAILURE_REASON_PAYMENT_FAILED |
| Payment accepted | No response from API | PaySuccess |
</details>

<details>
<summary>Examples</summary>

###### Full payment process with age verification (no errors)

> **->** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}

- *Age Verification on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {}}

> **->** {"goods_issued":{}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}

###### Full payment process with external age verification (AgeApproveRequest)

> **->** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}

> **->** {"ageApprove": {}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {}}

> **->** {"goods_issued":{}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}

###### Payment process without age verification (no errors, using pre-generated UUIDs)

> **->** {"id":{"msb":5, "lsb":0}, "start": {"amount": 100}}

> **\<-** {"id": {"msb": "5"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "5"}, "approved": {}}

> **->** {"id": {"msb":5, "lsb":0}, "goods_issued": {}}

> **\<-** {"id": {"msb": "5"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED" }}

> **\<-** {"id": {"msb": "5"}, "success": {}}

###### Payment process without age verification (payment failed)

> **->** {"start": {"amount": 220}}

> **\<-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (failed)

> **\<-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "failure": {"failureReason": "PAY_FAILURE_REASON_PAYMENT_FAILED"}}

###### Full payment process with age verification (age verification failed)

> **->** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageApiSuccess": {"reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED"}}

- *Age Verification on device* (failed)

> **\<-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageFailure": {"underAge": {}}}

###### Payment process with detailed invoice tracking (i.e. basket)

> **->** {"start": {"amount": 100}}

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED" }}

- *payment on device* (success)

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 0, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **\<-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "success": {}}

###### Payment process with invoice tracking change

> **->** {"start": {"amount": 100, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 60, "line_items": [{"price": 60, "quantity":1, "selection": {"slot":2}}] }}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

###### Mixed payment (cash paid before PaymentStart)
> **->** {"start": {"amount": 100, "cash_amount": 50}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {}}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

###### Mixed payment (cash amount changed before GoodsIssued)
> **->** {"start": {"amount": 100, "cash_amount": 50}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

###### Mixed payment (cash amount only known on GoodsIssued)
> **->** {"start": {"amount": 150}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

> **\<-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {}}

> **->** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}

> **\<-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **->**  {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}

</details>
*/
        async fn pay(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::api::v1::PayRequest>,
            >,
        ) -> std::result::Result<tonic::Response<Self::PayStream>, tonic::Status>;
    }
    /**
This service provides the necessary functionality to handle payments via the SENVEND Terminal.\
Optionally, age verification can be enforced before the payment via the `PayStart` message.\
Optionally, vending is possible after APPROVE is received, either via this or via the `Vend` service.
*/
    #[derive(Debug)]
    pub struct PayServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PayServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PayServiceServer<T>
    where
        T: PayService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/local.v1.PayService/Pay" => {
                    #[allow(non_camel_case_types)]
                    struct PaySvc<T: PayService>(pub Arc<T>);
                    impl<
                        T: PayService,
                    > tonic::server::StreamingService<
                        super::super::super::api::v1::PayRequest,
                    > for PaySvc<T> {
                        type Response = super::super::super::api::v1::PayResponse;
                        type ResponseStream = T::PayStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::api::v1::PayRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PayService>::pay(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PaySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for PayServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "local.v1.PayService";
    impl<T> tonic::server::NamedService for PayServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod vend_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /**
This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
*/
    #[derive(Debug, Clone)]
    pub struct VendServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VendServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VendServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VendServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            VendServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /**
Initiates a vending process on a machine connected to the SENVEND terminal.\
Accepts a stream of VendRequest for starting and controlling vending.\
Returns a stream of VendResponse containing status and error return messages.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given and a process is running, the request is applied to that running process.\
Otherwise a new UUID is generated per request.

- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.\
An empty id will work as well if the original request started the currently running process.

- All given ids must be valid version 4 UUIDs.

- If disconnected during a vending process, after a reconnect the currently running process can still be controlled.\
VendResponses that occurred during the disconnect are lost though.

- The minimum quantity to vend has to be 1, otherwise the request will be rejected.

- Items are vended via LineItem messages. The `price` field is optional and not necessary for vending.
</details>

<details open>
<summary>Process Constraints</summary>

- There can only be one vending process at a time. Multiple items can either be vended one-by-one,\
or by combining them all into one VendStart message.

- For each individual vending attempt, a VendEvent is sent back, indicating success or failure.

- For LineItems with a quantity greater than 1, items will be vended one-by-one until all are successful, or the FIRST vending failure.\
The resulting VendEvent failure message will also contain the amount of successfully vended items.

- If multiple LineItems are given, the list is vended according to the order of the LineItems in the message,
regardless of success or failure.

- Vending via this endpoint is also available when there is an ongoing payment,
specifically after a payment was APPROVED but before GOODS_ISSUED.\
If you don't need the UUIDs of this endpoint, consider using the VendStart messages of the `Pay` endpoint.

- Unlike the VendStart messages embedded within PayRequest, this API provides UUIDs per VendRequest.\
If you require precise control over the vending process, use this API to vend single items,
and match requests and answers via their UUIDs.

- The cancel request is provided to enable stopping midway during vending of a list of LineItems.\
If vending a single item, a cancel usually arrives too late to stop the process.
</details>
*/
        pub async fn vend(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::api::v1::VendRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::api::v1::VendResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/local.v1.VendService/Vend",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new("local.v1.VendService", "Vend"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vend_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VendServiceServer.
    #[async_trait]
    pub trait VendService: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the Vend method.
        type VendStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::api::v1::VendResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        /**
Initiates a vending process on a machine connected to the SENVEND terminal.\
Accepts a stream of VendRequest for starting and controlling vending.\
Returns a stream of VendResponse containing status and error return messages.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given and a process is running, the request is applied to that running process.\
Otherwise a new UUID is generated per request.

- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.\
An empty id will work as well if the original request started the currently running process.

- All given ids must be valid version 4 UUIDs.

- If disconnected during a vending process, after a reconnect the currently running process can still be controlled.\
VendResponses that occurred during the disconnect are lost though.

- The minimum quantity to vend has to be 1, otherwise the request will be rejected.

- Items are vended via LineItem messages. The `price` field is optional and not necessary for vending.
</details>

<details open>
<summary>Process Constraints</summary>

- There can only be one vending process at a time. Multiple items can either be vended one-by-one,\
or by combining them all into one VendStart message.

- For each individual vending attempt, a VendEvent is sent back, indicating success or failure.

- For LineItems with a quantity greater than 1, items will be vended one-by-one until all are successful, or the FIRST vending failure.\
The resulting VendEvent failure message will also contain the amount of successfully vended items.

- If multiple LineItems are given, the list is vended according to the order of the LineItems in the message,
regardless of success or failure.

- Vending via this endpoint is also available when there is an ongoing payment,
specifically after a payment was APPROVED but before GOODS_ISSUED.\
If you don't need the UUIDs of this endpoint, consider using the VendStart messages of the `Pay` endpoint.

- Unlike the VendStart messages embedded within PayRequest, this API provides UUIDs per VendRequest.\
If you require precise control over the vending process, use this API to vend single items,
and match requests and answers via their UUIDs.

- The cancel request is provided to enable stopping midway during vending of a list of LineItems.\
If vending a single item, a cancel usually arrives too late to stop the process.
</details>
*/
        async fn vend(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::api::v1::VendRequest>,
            >,
        ) -> std::result::Result<tonic::Response<Self::VendStream>, tonic::Status>;
    }
    /**
This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
*/
    #[derive(Debug)]
    pub struct VendServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VendServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VendServiceServer<T>
    where
        T: VendService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/local.v1.VendService/Vend" => {
                    #[allow(non_camel_case_types)]
                    struct VendSvc<T: VendService>(pub Arc<T>);
                    impl<
                        T: VendService,
                    > tonic::server::StreamingService<
                        super::super::super::api::v1::VendRequest,
                    > for VendSvc<T> {
                        type Response = super::super::super::api::v1::VendResponse;
                        type ResponseStream = T::VendStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::api::v1::VendRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VendService>::vend(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VendSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for VendServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "local.v1.VendService";
    impl<T> tonic::server::NamedService for VendServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod version_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /**
This service provides version information for the software on the SENVEND terminal.
*/
    #[derive(Debug, Clone)]
    pub struct VersionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VersionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VersionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VersionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            VersionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /**
Returns the version information of the software and API on the SENVEND terminal.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given a new UUID is generated per request.\
These are mostly provided for the cloud API functionality.
</details>

<details>
<summary>Examples</summary>

###### Standard version request

> **->** {}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, appVersion": {"major": 1, "minor": 3, "patch": 11}, "apiVersion": {"patch": 1}}
</details>
*/
        pub async fn version(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::api::v1::VersionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::api::v1::VersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/local.v1.VersionService/Version",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("local.v1.VersionService", "Version"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod version_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VersionServiceServer.
    #[async_trait]
    pub trait VersionService: std::marker::Send + std::marker::Sync + 'static {
        /**
Returns the version information of the software and API on the SENVEND terminal.

<details open>
<summary>API Constraints</summary>

- Request ids are optional.\
If none is given a new UUID is generated per request.\
These are mostly provided for the cloud API functionality.
</details>

<details>
<summary>Examples</summary>

###### Standard version request

> **->** {}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, appVersion": {"major": 1, "minor": 3, "patch": 11}, "apiVersion": {"patch": 1}}
</details>
*/
        async fn version(
            &self,
            request: tonic::Request<super::super::super::api::v1::VersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::api::v1::VersionResponse>,
            tonic::Status,
        >;
    }
    /**
This service provides version information for the software on the SENVEND terminal.
*/
    #[derive(Debug)]
    pub struct VersionServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VersionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VersionServiceServer<T>
    where
        T: VersionService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/local.v1.VersionService/Version" => {
                    #[allow(non_camel_case_types)]
                    struct VersionSvc<T: VersionService>(pub Arc<T>);
                    impl<
                        T: VersionService,
                    > tonic::server::UnaryService<
                        super::super::super::api::v1::VersionRequest,
                    > for VersionSvc<T> {
                        type Response = super::super::super::api::v1::VersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::api::v1::VersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VersionService>::version(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VersionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for VersionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "local.v1.VersionService";
    impl<T> tonic::server::NamedService for VersionServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
