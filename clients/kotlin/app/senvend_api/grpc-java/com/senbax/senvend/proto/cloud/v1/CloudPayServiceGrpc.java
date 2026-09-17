package com.senbax.senvend.proto.cloud.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * This service provides the necessary functionality to handle payments via the SENVEND Terminal.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CloudPayServiceGrpc {

  private CloudPayServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "cloud.v1.CloudPayService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudPayRequest,
      com.senbax.senvend.proto.cloud.v1.CloudPayResponse> getCloudPayMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CloudPay",
      requestType = com.senbax.senvend.proto.cloud.v1.CloudPayRequest.class,
      responseType = com.senbax.senvend.proto.cloud.v1.CloudPayResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
  public static io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudPayRequest,
      com.senbax.senvend.proto.cloud.v1.CloudPayResponse> getCloudPayMethod() {
    io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudPayRequest, com.senbax.senvend.proto.cloud.v1.CloudPayResponse> getCloudPayMethod;
    if ((getCloudPayMethod = CloudPayServiceGrpc.getCloudPayMethod) == null) {
      synchronized (CloudPayServiceGrpc.class) {
        if ((getCloudPayMethod = CloudPayServiceGrpc.getCloudPayMethod) == null) {
          CloudPayServiceGrpc.getCloudPayMethod = getCloudPayMethod =
              io.grpc.MethodDescriptor.<com.senbax.senvend.proto.cloud.v1.CloudPayRequest, com.senbax.senvend.proto.cloud.v1.CloudPayResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CloudPay"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.cloud.v1.CloudPayRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.cloud.v1.CloudPayResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CloudPayServiceMethodDescriptorSupplier("CloudPay"))
              .build();
        }
      }
    }
    return getCloudPayMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CloudPayServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceStub>() {
        @java.lang.Override
        public CloudPayServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudPayServiceStub(channel, callOptions);
        }
      };
    return CloudPayServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CloudPayServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceBlockingV2Stub>() {
        @java.lang.Override
        public CloudPayServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudPayServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return CloudPayServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CloudPayServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceBlockingStub>() {
        @java.lang.Override
        public CloudPayServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudPayServiceBlockingStub(channel, callOptions);
        }
      };
    return CloudPayServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CloudPayServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudPayServiceFutureStub>() {
        @java.lang.Override
        public CloudPayServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudPayServiceFutureStub(channel, callOptions);
        }
      };
    return CloudPayServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * This service provides the necessary functionality to handle payments via the SENVEND Terminal.
   * </pre>
   */
  public interface AsyncService {

    /**
     * <pre>
     * Initiates a payment process on the SENVEND Terminal.
     * </pre>
     */
    default io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudPayRequest> cloudPay(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudPayResponse> responseObserver) {
      return io.grpc.stub.ServerCalls.asyncUnimplementedStreamingCall(getCloudPayMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service CloudPayService.
   * <pre>
   * This service provides the necessary functionality to handle payments via the SENVEND Terminal.
   * </pre>
   */
  public static abstract class CloudPayServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CloudPayServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service CloudPayService.
   * <pre>
   * This service provides the necessary functionality to handle payments via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudPayServiceStub
      extends io.grpc.stub.AbstractAsyncStub<CloudPayServiceStub> {
    private CloudPayServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudPayServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudPayServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     * Initiates a payment process on the SENVEND Terminal.
     * </pre>
     */
    public io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudPayRequest> cloudPay(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudPayResponse> responseObserver) {
      return io.grpc.stub.ClientCalls.asyncBidiStreamingCall(
          getChannel().newCall(getCloudPayMethod(), getCallOptions()), responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service CloudPayService.
   * <pre>
   * This service provides the necessary functionality to handle payments via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudPayServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CloudPayServiceBlockingV2Stub> {
    private CloudPayServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudPayServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudPayServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     * <pre>
     * Initiates a payment process on the SENVEND Terminal.
     * </pre>
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<com.senbax.senvend.proto.cloud.v1.CloudPayRequest, com.senbax.senvend.proto.cloud.v1.CloudPayResponse>
        cloudPay() {
      return io.grpc.stub.ClientCalls.blockingBidiStreamingCall(
          getChannel(), getCloudPayMethod(), getCallOptions());
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service CloudPayService.
   * <pre>
   * This service provides the necessary functionality to handle payments via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudPayServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CloudPayServiceBlockingStub> {
    private CloudPayServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudPayServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudPayServiceBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service CloudPayService.
   * <pre>
   * This service provides the necessary functionality to handle payments via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudPayServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<CloudPayServiceFutureStub> {
    private CloudPayServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudPayServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudPayServiceFutureStub(channel, callOptions);
    }
  }

  private static final int METHODID_CLOUD_PAY = 0;

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
        case METHODID_CLOUD_PAY:
          return (io.grpc.stub.StreamObserver<Req>) serviceImpl.cloudPay(
              (io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudPayResponse>) responseObserver);
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getCloudPayMethod(),
          io.grpc.stub.ServerCalls.asyncBidiStreamingCall(
            new MethodHandlers<
              com.senbax.senvend.proto.cloud.v1.CloudPayRequest,
              com.senbax.senvend.proto.cloud.v1.CloudPayResponse>(
                service, METHODID_CLOUD_PAY)))
        .build();
  }

  private static abstract class CloudPayServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CloudPayServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.senbax.senvend.proto.cloud.v1.CloudProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("CloudPayService");
    }
  }

  private static final class CloudPayServiceFileDescriptorSupplier
      extends CloudPayServiceBaseDescriptorSupplier {
    CloudPayServiceFileDescriptorSupplier() {}
  }

  private static final class CloudPayServiceMethodDescriptorSupplier
      extends CloudPayServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CloudPayServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CloudPayServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CloudPayServiceFileDescriptorSupplier())
              .addMethod(getCloudPayMethod())
              .build();
        }
      }
    }
    return result;
  }
}
