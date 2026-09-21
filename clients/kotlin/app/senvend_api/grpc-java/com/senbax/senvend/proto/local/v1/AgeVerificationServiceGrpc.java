package com.senbax.senvend.proto.local.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 *This service provides the necessary functionality to handle age verification via the SENVEND Terminal.&#92;
 *Only necessary if age verification is the sole purpose.&#92;
 *The PayService contains its own way of starting age verification before the actual payment.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class AgeVerificationServiceGrpc {

  private AgeVerificationServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "local.v1.AgeVerificationService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.AgeRequest,
      com.senbax.senvend.proto.api.v1.AgeResponse> getAgeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Age",
      requestType = com.senbax.senvend.proto.api.v1.AgeRequest.class,
      responseType = com.senbax.senvend.proto.api.v1.AgeResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
  public static io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.AgeRequest,
      com.senbax.senvend.proto.api.v1.AgeResponse> getAgeMethod() {
    io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.AgeRequest, com.senbax.senvend.proto.api.v1.AgeResponse> getAgeMethod;
    if ((getAgeMethod = AgeVerificationServiceGrpc.getAgeMethod) == null) {
      synchronized (AgeVerificationServiceGrpc.class) {
        if ((getAgeMethod = AgeVerificationServiceGrpc.getAgeMethod) == null) {
          AgeVerificationServiceGrpc.getAgeMethod = getAgeMethod =
              io.grpc.MethodDescriptor.<com.senbax.senvend.proto.api.v1.AgeRequest, com.senbax.senvend.proto.api.v1.AgeResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Age"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.AgeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.AgeResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AgeVerificationServiceMethodDescriptorSupplier("Age"))
              .build();
        }
      }
    }
    return getAgeMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static AgeVerificationServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceStub>() {
        @java.lang.Override
        public AgeVerificationServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AgeVerificationServiceStub(channel, callOptions);
        }
      };
    return AgeVerificationServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static AgeVerificationServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceBlockingV2Stub>() {
        @java.lang.Override
        public AgeVerificationServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AgeVerificationServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return AgeVerificationServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static AgeVerificationServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceBlockingStub>() {
        @java.lang.Override
        public AgeVerificationServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AgeVerificationServiceBlockingStub(channel, callOptions);
        }
      };
    return AgeVerificationServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static AgeVerificationServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AgeVerificationServiceFutureStub>() {
        @java.lang.Override
        public AgeVerificationServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AgeVerificationServiceFutureStub(channel, callOptions);
        }
      };
    return AgeVerificationServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   *This service provides the necessary functionality to handle age verification via the SENVEND Terminal.&#92;
   *Only necessary if age verification is the sole purpose.&#92;
   *The PayService contains its own way of starting age verification before the actual payment.
   * </pre>
   */
  public interface AsyncService {

    /**
     * <pre>
     *Initiates an age verification process on the SENVEND terminal.&#92;
     *Accepts a stream of AgeRequest for starting and controlling age verification.&#92;
     *Returns a stream of AgeResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a verification process, after a reconnect the currently running process can still be controlled.&#92;
     *AgeResponses that occurred during the disconnect are lost though.
     *- If auto_cancel is left out or set to true, a new AgeRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.&#92;
     *If given but false, sending a new AgeRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- The minimum age to verify has to be greater than zero and can maximally be 120.&#92;
     *Depending on the method chosen, only certain ages can be verified.
     *- Girocard: 16 or 18. Will be set to the next one above if below (e.g. to 18 if 17 is requested).&#92;
     *Will not be available for selection if above 18 is requested. Will fail in older app versions (&lt;1.7.0) instead.
     *- FaceScan: All ages supported. Might fail if actual age is very close to the requested one.
     *- Document scan: All ages supported.
     *- PayPal: 18 only. Will be set to 18 if below.&#92;
     *Will not be available for selection if above 18 is requested. Will fail in older app versions (&lt;1.7.0) instead.
     *- ID Austria: All ages supported.
     *&lt;/details&gt;
     * </pre>
     */
    default io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.AgeRequest> age(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.AgeResponse> responseObserver) {
      return io.grpc.stub.ServerCalls.asyncUnimplementedStreamingCall(getAgeMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service AgeVerificationService.
   * <pre>
   *This service provides the necessary functionality to handle age verification via the SENVEND Terminal.&#92;
   *Only necessary if age verification is the sole purpose.&#92;
   *The PayService contains its own way of starting age verification before the actual payment.
   * </pre>
   */
  public static abstract class AgeVerificationServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return AgeVerificationServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service AgeVerificationService.
   * <pre>
   *This service provides the necessary functionality to handle age verification via the SENVEND Terminal.&#92;
   *Only necessary if age verification is the sole purpose.&#92;
   *The PayService contains its own way of starting age verification before the actual payment.
   * </pre>
   */
  public static final class AgeVerificationServiceStub
      extends io.grpc.stub.AbstractAsyncStub<AgeVerificationServiceStub> {
    private AgeVerificationServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AgeVerificationServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AgeVerificationServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     *Initiates an age verification process on the SENVEND terminal.&#92;
     *Accepts a stream of AgeRequest for starting and controlling age verification.&#92;
     *Returns a stream of AgeResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a verification process, after a reconnect the currently running process can still be controlled.&#92;
     *AgeResponses that occurred during the disconnect are lost though.
     *- If auto_cancel is left out or set to true, a new AgeRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.&#92;
     *If given but false, sending a new AgeRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- The minimum age to verify has to be greater than zero and can maximally be 120.&#92;
     *Depending on the method chosen, only certain ages can be verified.
     *- Girocard: 16 or 18. Will be set to the next one above if below (e.g. to 18 if 17 is requested).&#92;
     *Will not be available for selection if above 18 is requested. Will fail in older app versions (&lt;1.7.0) instead.
     *- FaceScan: All ages supported. Might fail if actual age is very close to the requested one.
     *- Document scan: All ages supported.
     *- PayPal: 18 only. Will be set to 18 if below.&#92;
     *Will not be available for selection if above 18 is requested. Will fail in older app versions (&lt;1.7.0) instead.
     *- ID Austria: All ages supported.
     *&lt;/details&gt;
     * </pre>
     */
    public io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.AgeRequest> age(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.AgeResponse> responseObserver) {
      return io.grpc.stub.ClientCalls.asyncBidiStreamingCall(
          getChannel().newCall(getAgeMethod(), getCallOptions()), responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service AgeVerificationService.
   * <pre>
   *This service provides the necessary functionality to handle age verification via the SENVEND Terminal.&#92;
   *Only necessary if age verification is the sole purpose.&#92;
   *The PayService contains its own way of starting age verification before the actual payment.
   * </pre>
   */
  public static final class AgeVerificationServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<AgeVerificationServiceBlockingV2Stub> {
    private AgeVerificationServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AgeVerificationServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AgeVerificationServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     * <pre>
     *Initiates an age verification process on the SENVEND terminal.&#92;
     *Accepts a stream of AgeRequest for starting and controlling age verification.&#92;
     *Returns a stream of AgeResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a verification process, after a reconnect the currently running process can still be controlled.&#92;
     *AgeResponses that occurred during the disconnect are lost though.
     *- If auto_cancel is left out or set to true, a new AgeRequest with a different UUID or without one will automatically cancel any currently running process on the terminal.&#92;
     *If given but false, sending a new AgeRequest while another process is still running will result in a PAY_FAILURE_REASON_PAYMENT_ONGOING error message.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- The minimum age to verify has to be greater than zero and can maximally be 120.&#92;
     *Depending on the method chosen, only certain ages can be verified.
     *- Girocard: 16 or 18. Will be set to the next one above if below (e.g. to 18 if 17 is requested).&#92;
     *Will not be available for selection if above 18 is requested. Will fail in older app versions (&lt;1.7.0) instead.
     *- FaceScan: All ages supported. Might fail if actual age is very close to the requested one.
     *- Document scan: All ages supported.
     *- PayPal: 18 only. Will be set to 18 if below.&#92;
     *Will not be available for selection if above 18 is requested. Will fail in older app versions (&lt;1.7.0) instead.
     *- ID Austria: All ages supported.
     *&lt;/details&gt;
     * </pre>
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<com.senbax.senvend.proto.api.v1.AgeRequest, com.senbax.senvend.proto.api.v1.AgeResponse>
        age() {
      return io.grpc.stub.ClientCalls.blockingBidiStreamingCall(
          getChannel(), getAgeMethod(), getCallOptions());
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service AgeVerificationService.
   * <pre>
   *This service provides the necessary functionality to handle age verification via the SENVEND Terminal.&#92;
   *Only necessary if age verification is the sole purpose.&#92;
   *The PayService contains its own way of starting age verification before the actual payment.
   * </pre>
   */
  public static final class AgeVerificationServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<AgeVerificationServiceBlockingStub> {
    private AgeVerificationServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AgeVerificationServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AgeVerificationServiceBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service AgeVerificationService.
   * <pre>
   *This service provides the necessary functionality to handle age verification via the SENVEND Terminal.&#92;
   *Only necessary if age verification is the sole purpose.&#92;
   *The PayService contains its own way of starting age verification before the actual payment.
   * </pre>
   */
  public static final class AgeVerificationServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<AgeVerificationServiceFutureStub> {
    private AgeVerificationServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AgeVerificationServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AgeVerificationServiceFutureStub(channel, callOptions);
    }
  }

  private static final int METHODID_AGE = 0;

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
        case METHODID_AGE:
          return (io.grpc.stub.StreamObserver<Req>) serviceImpl.age(
              (io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.AgeResponse>) responseObserver);
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getAgeMethod(),
          io.grpc.stub.ServerCalls.asyncBidiStreamingCall(
            new MethodHandlers<
              com.senbax.senvend.proto.api.v1.AgeRequest,
              com.senbax.senvend.proto.api.v1.AgeResponse>(
                service, METHODID_AGE)))
        .build();
  }

  private static abstract class AgeVerificationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    AgeVerificationServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.senbax.senvend.proto.local.v1.LocalProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("AgeVerificationService");
    }
  }

  private static final class AgeVerificationServiceFileDescriptorSupplier
      extends AgeVerificationServiceBaseDescriptorSupplier {
    AgeVerificationServiceFileDescriptorSupplier() {}
  }

  private static final class AgeVerificationServiceMethodDescriptorSupplier
      extends AgeVerificationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    AgeVerificationServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (AgeVerificationServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new AgeVerificationServiceFileDescriptorSupplier())
              .addMethod(getAgeMethod())
              .build();
        }
      }
    }
    return result;
  }
}
