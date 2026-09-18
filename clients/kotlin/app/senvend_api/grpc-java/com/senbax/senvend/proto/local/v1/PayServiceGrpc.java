package com.senbax.senvend.proto.local.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 *This service provides the necessary functionality to handle payments via the SENVEND Terminal.&#92;
 *Optionally, age verification can be enforced before the payment via the `PayStart` message.&#92;
 *Optionally, vending is possible after PayApproved is received, either via this or via the `Vend` service.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class PayServiceGrpc {

  private PayServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "local.v1.PayService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.PayRequest,
      com.senbax.senvend.proto.api.v1.PayResponse> getPayMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Pay",
      requestType = com.senbax.senvend.proto.api.v1.PayRequest.class,
      responseType = com.senbax.senvend.proto.api.v1.PayResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
  public static io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.PayRequest,
      com.senbax.senvend.proto.api.v1.PayResponse> getPayMethod() {
    io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.PayRequest, com.senbax.senvend.proto.api.v1.PayResponse> getPayMethod;
    if ((getPayMethod = PayServiceGrpc.getPayMethod) == null) {
      synchronized (PayServiceGrpc.class) {
        if ((getPayMethod = PayServiceGrpc.getPayMethod) == null) {
          PayServiceGrpc.getPayMethod = getPayMethod =
              io.grpc.MethodDescriptor.<com.senbax.senvend.proto.api.v1.PayRequest, com.senbax.senvend.proto.api.v1.PayResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Pay"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.PayRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.PayResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PayServiceMethodDescriptorSupplier("Pay"))
              .build();
        }
      }
    }
    return getPayMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PayServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayServiceStub>() {
        @java.lang.Override
        public PayServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayServiceStub(channel, callOptions);
        }
      };
    return PayServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static PayServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayServiceBlockingV2Stub>() {
        @java.lang.Override
        public PayServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return PayServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PayServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayServiceBlockingStub>() {
        @java.lang.Override
        public PayServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayServiceBlockingStub(channel, callOptions);
        }
      };
    return PayServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PayServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PayServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PayServiceFutureStub>() {
        @java.lang.Override
        public PayServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PayServiceFutureStub(channel, callOptions);
        }
      };
    return PayServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   *This service provides the necessary functionality to handle payments via the SENVEND Terminal.&#92;
   *Optionally, age verification can be enforced before the payment via the `PayStart` message.&#92;
   *Optionally, vending is possible after PayApproved is received, either via this or via the `Vend` service.
   * </pre>
   */
  public interface AsyncService {

    /**
     * <pre>
     *Initiates a payment process on the SENVEND Terminal.&#92;
     *Accepts a stream of PayRequest for starting and controlling payments.&#92;
     *Returns a stream of PayResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a payment process, after a reconnect the currently running process can still be controlled.&#92;
     *PayResponses that occurred during the disconnect are lost though.
     *- If auto_cancel is left out or set to true, a new PayRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.&#92;
     *If given but false, sending a new PayRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- The amount to charge is given in cents and can even be zero.&#92;
     *The last option is useful to combine vending or age verification with a `PayGoodsIssued` message,&#92;
     *mostly for telemetry purposes.
     *- The minimum age to verify has to be greater than zero and at most 120.&#92;
     *Depending on the method chosen, only certain ages can be verified.&#92;
     *See the `Age` service for more details.
     *- An additional external age verification step can be implemented by sending an `AgeApproveRequest` message.&#92;
     *This will mark the age verification as approved and continue with payment.
     *- If a payment was approved, a `PayGoodsIssued` message must be sent in order to finalize it.&#92;
     **The client has 9m30s to answer to the approval, or the goods will be issued to the customer as an emergency measure.**
     *- Vending can also be done via this endpoint by sending a VendStart message.&#92;
     *These are accepted either when no payment is running, or after the payment was APPROVED and before sending GOODS_ISSUED.&#92;
     *See `Vend` service for details.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Telemetry / Invoice Line Items / Mixed Payments&lt;/summary&gt;
     *It is possible to send a list of products, their prices and the quantity per product sold alongside the `pay_start` and `goods_issued` requests. See the documentation of the api.v1.LineItem message.
     *Mixed payments can be supported by sending an additional cash_amount via the `pay_start` or `goods_issued` message.
     *The amount in PayStart.amount or PayGoodsIssued.partial_amount only covers cashless transactions,
     *therefore cash_amount is independent of that and only for reporting purposes via telemetry.
     *The device will do a verification of the payment amount (including cash_amount if present) versus the sum of the provided LineItem list, and report a PAY_API_FAILURE_REASON_AMOUNT_MISMATCH if these amounts mismatch.
     *These messages are processed by the SENVEND web portal and taken into consideration when generating sales reports.
     *If line_items or cash_amount are sent alongside the `goods_issued` message, they take precedence over any values from the `pay_start` message, effectively overriding them.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;State and state changes&lt;/summary&gt;
     *| State | Request | Result |
     *|--------------------------|----------------------------|------------------------------|
     *| No payment running | PayStart | Payment start |
     *| | PayStart (with AgeRequest) | AgeVerification start |
     *| | PayCancel | PayApiFailure OR PayApiSuccess if sent without UUID |
     *| | PayGoodsIssued | PayApiFailure |
     *| Age verification ongoing | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | AgeVerification cancel |
     *| | PayGoodsIssued | PayApiFailure |
     *| | AgeApproveRequest | Terminal Proceeds to payment |
     *| Payment process ongoing | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | Payment cancel |
     *| | PayGoodsIssued | PayApiFailure |
     *| | AgeApproveRequest | PayApiFailure |
     *| Payment accepted | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | Reimburse and cancel payment |
     *| | PayGoodsIssued | Finalize payment |
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;State and errors&lt;/summary&gt;
     *| State | Event | Error |
     *|--------------------------|---------------------------|-----------------------------------|
     *| Age verification ongoing | Took too long (timeout) | AGE_FAILURE_REASON_TIMEOUT |
     *| | User actively canceled | AGE_FAILURE_REASON_USER_CANCELLED |
     *| | Cancel via API | AGE_FAILURE_REASON_API_CANCELLED |
     *| | User walked away | AGE_FAILURE_REASON_TIMEOUT |
     *| | Verification fails | AgeFailureUnderage |
     *| Payment process ongoing | Took too long (timeout) | PAY_FAILURE_REASON_TIMEOUT |
     *| | User actively canceled | PAY_FAILURE_REASON_USER_CANCELLED |
     *| | Canceled via API | PAY_FAILURE_REASON_API_CANCELLED |
     *| | Payment failed (no debit) | PAY_FAILURE_REASON_PAYMENT_FAILED |
     *| | User walked away | PAY_FAILURE_REASON_TIMEOUT |
     *| Payment accepted | PayGoodsIssued | PaySuccess |
     *| | No Response from API | PaySuccess |
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Full payment process with age verification (no errors)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}
     *- *Age Verification on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued":{}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}
     *###### Full payment process with external age verification (AgeApproveRequest)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}
     *&gt; **-&gt;** {"ageApprove": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued":{}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}
     *###### Payment process without age verification (no errors, using pre-generated UUIDs)
     *&gt; **-&gt;** {"id":{"msb":5, "lsb":0}, "start": {"amount": 100}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"id": {"msb":5, "lsb":0}, "goods_issued": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED" }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "success": {}}
     *###### Payment process without age verification (payment failed)
     *&gt; **-&gt;** {"start": {"amount": 220}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (failed)
     *&gt; **&#92;&lt;-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "failure": {"failureReason": "PAY_FAILURE_REASON_PAYMENT_FAILED"}}
     *###### Full payment process with age verification (age verification failed)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageApiSuccess": {"reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED"}}
     *- *Age Verification on device* (failed)
     *&gt; **&#92;&lt;-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageFailure": {"underAge": {}}}
     *###### Payment process with detailed invoice tracking (i.e. basket)
     *&gt; **-&gt;** {"start": {"amount": 100}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED" }}
     *- *payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 0, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "success": {}}
     *###### Payment process with invoice tracking change
     *&gt; **-&gt;** {"start": {"amount": 100, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 60, "line_items": [{"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash paid before PayStart)
     *&gt; **-&gt;** {"start": {"amount": 100, "cash_amount": 50}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash amount changed before PayGoodsIssued)
     *&gt; **-&gt;** {"start": {"amount": 100, "cash_amount": 50}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash amount only known on PayGoodsIssued)
     *&gt; **-&gt;** {"start": {"amount": 150}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 150}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *&lt;/details&gt;
     * </pre>
     */
    default io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.PayRequest> pay(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.PayResponse> responseObserver) {
      return io.grpc.stub.ServerCalls.asyncUnimplementedStreamingCall(getPayMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service PayService.
   * <pre>
   *This service provides the necessary functionality to handle payments via the SENVEND Terminal.&#92;
   *Optionally, age verification can be enforced before the payment via the `PayStart` message.&#92;
   *Optionally, vending is possible after PayApproved is received, either via this or via the `Vend` service.
   * </pre>
   */
  public static abstract class PayServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PayServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service PayService.
   * <pre>
   *This service provides the necessary functionality to handle payments via the SENVEND Terminal.&#92;
   *Optionally, age verification can be enforced before the payment via the `PayStart` message.&#92;
   *Optionally, vending is possible after PayApproved is received, either via this or via the `Vend` service.
   * </pre>
   */
  public static final class PayServiceStub
      extends io.grpc.stub.AbstractAsyncStub<PayServiceStub> {
    private PayServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     *Initiates a payment process on the SENVEND Terminal.&#92;
     *Accepts a stream of PayRequest for starting and controlling payments.&#92;
     *Returns a stream of PayResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a payment process, after a reconnect the currently running process can still be controlled.&#92;
     *PayResponses that occurred during the disconnect are lost though.
     *- If auto_cancel is left out or set to true, a new PayRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.&#92;
     *If given but false, sending a new PayRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- The amount to charge is given in cents and can even be zero.&#92;
     *The last option is useful to combine vending or age verification with a `PayGoodsIssued` message,&#92;
     *mostly for telemetry purposes.
     *- The minimum age to verify has to be greater than zero and at most 120.&#92;
     *Depending on the method chosen, only certain ages can be verified.&#92;
     *See the `Age` service for more details.
     *- An additional external age verification step can be implemented by sending an `AgeApproveRequest` message.&#92;
     *This will mark the age verification as approved and continue with payment.
     *- If a payment was approved, a `PayGoodsIssued` message must be sent in order to finalize it.&#92;
     **The client has 9m30s to answer to the approval, or the goods will be issued to the customer as an emergency measure.**
     *- Vending can also be done via this endpoint by sending a VendStart message.&#92;
     *These are accepted either when no payment is running, or after the payment was APPROVED and before sending GOODS_ISSUED.&#92;
     *See `Vend` service for details.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Telemetry / Invoice Line Items / Mixed Payments&lt;/summary&gt;
     *It is possible to send a list of products, their prices and the quantity per product sold alongside the `pay_start` and `goods_issued` requests. See the documentation of the api.v1.LineItem message.
     *Mixed payments can be supported by sending an additional cash_amount via the `pay_start` or `goods_issued` message.
     *The amount in PayStart.amount or PayGoodsIssued.partial_amount only covers cashless transactions,
     *therefore cash_amount is independent of that and only for reporting purposes via telemetry.
     *The device will do a verification of the payment amount (including cash_amount if present) versus the sum of the provided LineItem list, and report a PAY_API_FAILURE_REASON_AMOUNT_MISMATCH if these amounts mismatch.
     *These messages are processed by the SENVEND web portal and taken into consideration when generating sales reports.
     *If line_items or cash_amount are sent alongside the `goods_issued` message, they take precedence over any values from the `pay_start` message, effectively overriding them.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;State and state changes&lt;/summary&gt;
     *| State | Request | Result |
     *|--------------------------|----------------------------|------------------------------|
     *| No payment running | PayStart | Payment start |
     *| | PayStart (with AgeRequest) | AgeVerification start |
     *| | PayCancel | PayApiFailure OR PayApiSuccess if sent without UUID |
     *| | PayGoodsIssued | PayApiFailure |
     *| Age verification ongoing | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | AgeVerification cancel |
     *| | PayGoodsIssued | PayApiFailure |
     *| | AgeApproveRequest | Terminal Proceeds to payment |
     *| Payment process ongoing | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | Payment cancel |
     *| | PayGoodsIssued | PayApiFailure |
     *| | AgeApproveRequest | PayApiFailure |
     *| Payment accepted | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | Reimburse and cancel payment |
     *| | PayGoodsIssued | Finalize payment |
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;State and errors&lt;/summary&gt;
     *| State | Event | Error |
     *|--------------------------|---------------------------|-----------------------------------|
     *| Age verification ongoing | Took too long (timeout) | AGE_FAILURE_REASON_TIMEOUT |
     *| | User actively canceled | AGE_FAILURE_REASON_USER_CANCELLED |
     *| | Cancel via API | AGE_FAILURE_REASON_API_CANCELLED |
     *| | User walked away | AGE_FAILURE_REASON_TIMEOUT |
     *| | Verification fails | AgeFailureUnderage |
     *| Payment process ongoing | Took too long (timeout) | PAY_FAILURE_REASON_TIMEOUT |
     *| | User actively canceled | PAY_FAILURE_REASON_USER_CANCELLED |
     *| | Canceled via API | PAY_FAILURE_REASON_API_CANCELLED |
     *| | Payment failed (no debit) | PAY_FAILURE_REASON_PAYMENT_FAILED |
     *| | User walked away | PAY_FAILURE_REASON_TIMEOUT |
     *| Payment accepted | PayGoodsIssued | PaySuccess |
     *| | No Response from API | PaySuccess |
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Full payment process with age verification (no errors)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}
     *- *Age Verification on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued":{}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}
     *###### Full payment process with external age verification (AgeApproveRequest)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}
     *&gt; **-&gt;** {"ageApprove": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued":{}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}
     *###### Payment process without age verification (no errors, using pre-generated UUIDs)
     *&gt; **-&gt;** {"id":{"msb":5, "lsb":0}, "start": {"amount": 100}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"id": {"msb":5, "lsb":0}, "goods_issued": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED" }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "success": {}}
     *###### Payment process without age verification (payment failed)
     *&gt; **-&gt;** {"start": {"amount": 220}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (failed)
     *&gt; **&#92;&lt;-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "failure": {"failureReason": "PAY_FAILURE_REASON_PAYMENT_FAILED"}}
     *###### Full payment process with age verification (age verification failed)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageApiSuccess": {"reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED"}}
     *- *Age Verification on device* (failed)
     *&gt; **&#92;&lt;-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageFailure": {"underAge": {}}}
     *###### Payment process with detailed invoice tracking (i.e. basket)
     *&gt; **-&gt;** {"start": {"amount": 100}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED" }}
     *- *payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 0, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "success": {}}
     *###### Payment process with invoice tracking change
     *&gt; **-&gt;** {"start": {"amount": 100, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 60, "line_items": [{"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash paid before PayStart)
     *&gt; **-&gt;** {"start": {"amount": 100, "cash_amount": 50}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash amount changed before PayGoodsIssued)
     *&gt; **-&gt;** {"start": {"amount": 100, "cash_amount": 50}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash amount only known on PayGoodsIssued)
     *&gt; **-&gt;** {"start": {"amount": 150}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 150}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *&lt;/details&gt;
     * </pre>
     */
    public io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.PayRequest> pay(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.PayResponse> responseObserver) {
      return io.grpc.stub.ClientCalls.asyncBidiStreamingCall(
          getChannel().newCall(getPayMethod(), getCallOptions()), responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service PayService.
   * <pre>
   *This service provides the necessary functionality to handle payments via the SENVEND Terminal.&#92;
   *Optionally, age verification can be enforced before the payment via the `PayStart` message.&#92;
   *Optionally, vending is possible after PayApproved is received, either via this or via the `Vend` service.
   * </pre>
   */
  public static final class PayServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<PayServiceBlockingV2Stub> {
    private PayServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     * <pre>
     *Initiates a payment process on the SENVEND Terminal.&#92;
     *Accepts a stream of PayRequest for starting and controlling payments.&#92;
     *Returns a stream of PayResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a payment process, after a reconnect the currently running process can still be controlled.&#92;
     *PayResponses that occurred during the disconnect are lost though.
     *- If auto_cancel is left out or set to true, a new PayRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.&#92;
     *If given but false, sending a new PayRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- The amount to charge is given in cents and can even be zero.&#92;
     *The last option is useful to combine vending or age verification with a `PayGoodsIssued` message,&#92;
     *mostly for telemetry purposes.
     *- The minimum age to verify has to be greater than zero and at most 120.&#92;
     *Depending on the method chosen, only certain ages can be verified.&#92;
     *See the `Age` service for more details.
     *- An additional external age verification step can be implemented by sending an `AgeApproveRequest` message.&#92;
     *This will mark the age verification as approved and continue with payment.
     *- If a payment was approved, a `PayGoodsIssued` message must be sent in order to finalize it.&#92;
     **The client has 9m30s to answer to the approval, or the goods will be issued to the customer as an emergency measure.**
     *- Vending can also be done via this endpoint by sending a VendStart message.&#92;
     *These are accepted either when no payment is running, or after the payment was APPROVED and before sending GOODS_ISSUED.&#92;
     *See `Vend` service for details.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Telemetry / Invoice Line Items / Mixed Payments&lt;/summary&gt;
     *It is possible to send a list of products, their prices and the quantity per product sold alongside the `pay_start` and `goods_issued` requests. See the documentation of the api.v1.LineItem message.
     *Mixed payments can be supported by sending an additional cash_amount via the `pay_start` or `goods_issued` message.
     *The amount in PayStart.amount or PayGoodsIssued.partial_amount only covers cashless transactions,
     *therefore cash_amount is independent of that and only for reporting purposes via telemetry.
     *The device will do a verification of the payment amount (including cash_amount if present) versus the sum of the provided LineItem list, and report a PAY_API_FAILURE_REASON_AMOUNT_MISMATCH if these amounts mismatch.
     *These messages are processed by the SENVEND web portal and taken into consideration when generating sales reports.
     *If line_items or cash_amount are sent alongside the `goods_issued` message, they take precedence over any values from the `pay_start` message, effectively overriding them.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;State and state changes&lt;/summary&gt;
     *| State | Request | Result |
     *|--------------------------|----------------------------|------------------------------|
     *| No payment running | PayStart | Payment start |
     *| | PayStart (with AgeRequest) | AgeVerification start |
     *| | PayCancel | PayApiFailure OR PayApiSuccess if sent without UUID |
     *| | PayGoodsIssued | PayApiFailure |
     *| Age verification ongoing | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | AgeVerification cancel |
     *| | PayGoodsIssued | PayApiFailure |
     *| | AgeApproveRequest | Terminal Proceeds to payment |
     *| Payment process ongoing | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | Payment cancel |
     *| | PayGoodsIssued | PayApiFailure |
     *| | AgeApproveRequest | PayApiFailure |
     *| Payment accepted | PayStart | PayFailure (if auto_cancel=true, also PaymentStart) |
     *| | PayCancel | Reimburse and cancel payment |
     *| | PayGoodsIssued | Finalize payment |
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;State and errors&lt;/summary&gt;
     *| State | Event | Error |
     *|--------------------------|---------------------------|-----------------------------------|
     *| Age verification ongoing | Took too long (timeout) | AGE_FAILURE_REASON_TIMEOUT |
     *| | User actively canceled | AGE_FAILURE_REASON_USER_CANCELLED |
     *| | Cancel via API | AGE_FAILURE_REASON_API_CANCELLED |
     *| | User walked away | AGE_FAILURE_REASON_TIMEOUT |
     *| | Verification fails | AgeFailureUnderage |
     *| Payment process ongoing | Took too long (timeout) | PAY_FAILURE_REASON_TIMEOUT |
     *| | User actively canceled | PAY_FAILURE_REASON_USER_CANCELLED |
     *| | Canceled via API | PAY_FAILURE_REASON_API_CANCELLED |
     *| | Payment failed (no debit) | PAY_FAILURE_REASON_PAYMENT_FAILED |
     *| | User walked away | PAY_FAILURE_REASON_TIMEOUT |
     *| Payment accepted | PayGoodsIssued | PaySuccess |
     *| | No Response from API | PaySuccess |
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Full payment process with age verification (no errors)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}
     *- *Age Verification on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued":{}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}
     *###### Full payment process with external age verification (AgeApproveRequest)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}
     *&gt; **-&gt;** {"ageApprove": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued":{}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}
     *###### Payment process without age verification (no errors, using pre-generated UUIDs)
     *&gt; **-&gt;** {"id":{"msb":5, "lsb":0}, "start": {"amount": 100}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"id": {"msb":5, "lsb":0}, "goods_issued": {}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED" }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "5"}, "success": {}}
     *###### Payment process without age verification (payment failed)
     *&gt; **-&gt;** {"start": {"amount": 220}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *- *Payment on device* (failed)
     *&gt; **&#92;&lt;-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "failure": {"failureReason": "PAY_FAILURE_REASON_PAYMENT_FAILED"}}
     *###### Full payment process with age verification (age verification failed)
     *&gt; **-&gt;** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageApiSuccess": {"reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED"}}
     *- *Age Verification on device* (failed)
     *&gt; **&#92;&lt;-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageFailure": {"underAge": {}}}
     *###### Payment process with detailed invoice tracking (i.e. basket)
     *&gt; **-&gt;** {"start": {"amount": 100}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED" }}
     *- *payment on device* (success)
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 0, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "18075966312244266714", "lsb": "9624948136777214489"}, "success": {}}
     *###### Payment process with invoice tracking change
     *&gt; **-&gt;** {"start": {"amount": 100, "line_items": [{"price": 40, "quantity":1, "selection": {"slot":1}}, {"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 60, "line_items": [{"price": 60, "quantity":1, "selection": {"slot":2}}] }}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash paid before PayStart)
     *&gt; **-&gt;** {"start": {"amount": 100, "cash_amount": 50}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash amount changed before PayGoodsIssued)
     *&gt; **-&gt;** {"start": {"amount": 100, "cash_amount": 50}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 100}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *###### Mixed payment (cash amount only known on PayGoodsIssued)
     *&gt; **-&gt;** {"start": {"amount": 150}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "approved": {"amount": 150}}
     *&gt; **-&gt;** {"goods_issued": {"partial_amount": 50, "cash_amount": 100}}
     *&gt; **&#92;&lt;-** {"id": { "msb": "12005064334431440106", "lsb": "9545526647834091413"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}
     *&gt; **&#92;&lt;-** {"id": {"msb": "12005064334431440106", "lsb": "9545526647834091413"}, "success": {}}
     *&lt;/details&gt;
     * </pre>
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<com.senbax.senvend.proto.api.v1.PayRequest, com.senbax.senvend.proto.api.v1.PayResponse>
        pay() {
      return io.grpc.stub.ClientCalls.blockingBidiStreamingCall(
          getChannel(), getPayMethod(), getCallOptions());
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service PayService.
   * <pre>
   *This service provides the necessary functionality to handle payments via the SENVEND Terminal.&#92;
   *Optionally, age verification can be enforced before the payment via the `PayStart` message.&#92;
   *Optionally, vending is possible after PayApproved is received, either via this or via the `Vend` service.
   * </pre>
   */
  public static final class PayServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PayServiceBlockingStub> {
    private PayServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayServiceBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service PayService.
   * <pre>
   *This service provides the necessary functionality to handle payments via the SENVEND Terminal.&#92;
   *Optionally, age verification can be enforced before the payment via the `PayStart` message.&#92;
   *Optionally, vending is possible after PayApproved is received, either via this or via the `Vend` service.
   * </pre>
   */
  public static final class PayServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<PayServiceFutureStub> {
    private PayServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PayServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PayServiceFutureStub(channel, callOptions);
    }
  }

  private static final int METHODID_PAY = 0;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final AsyncService serviceImpl;
    private final int methodId;

    MethodHandlers(AsyncService serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_PAY:
          return (io.grpc.stub.StreamObserver<Req>) serviceImpl.pay(
              (io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.PayResponse>) responseObserver);
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getPayMethod(),
          io.grpc.stub.ServerCalls.asyncBidiStreamingCall(
            new MethodHandlers<
              com.senbax.senvend.proto.api.v1.PayRequest,
              com.senbax.senvend.proto.api.v1.PayResponse>(
                service, METHODID_PAY)))
        .build();
  }

  private static abstract class PayServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PayServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.senbax.senvend.proto.local.v1.LocalProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("PayService");
    }
  }

  private static final class PayServiceFileDescriptorSupplier
      extends PayServiceBaseDescriptorSupplier {
    PayServiceFileDescriptorSupplier() {}
  }

  private static final class PayServiceMethodDescriptorSupplier
      extends PayServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PayServiceMethodDescriptorSupplier(java.lang.String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (PayServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PayServiceFileDescriptorSupplier())
              .addMethod(getPayMethod())
              .build();
        }
      }
    }
    return result;
  }
}
