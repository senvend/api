package com.senbax.senvend.proto.cloud.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * This service provides the necessary functionality to handle age verification via the SENVEND Terminal.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CloudAgeVerificationServiceGrpc {

  private CloudAgeVerificationServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "cloud.v1.CloudAgeVerificationService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudAgeRequest,
      com.senbax.senvend.proto.cloud.v1.CloudAgeResponse> getCloudAgeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CloudAge",
      requestType = com.senbax.senvend.proto.cloud.v1.CloudAgeRequest.class,
      responseType = com.senbax.senvend.proto.cloud.v1.CloudAgeResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
  public static io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudAgeRequest,
      com.senbax.senvend.proto.cloud.v1.CloudAgeResponse> getCloudAgeMethod() {
    io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudAgeRequest, com.senbax.senvend.proto.cloud.v1.CloudAgeResponse> getCloudAgeMethod;
    if ((getCloudAgeMethod = CloudAgeVerificationServiceGrpc.getCloudAgeMethod) == null) {
      synchronized (CloudAgeVerificationServiceGrpc.class) {
        if ((getCloudAgeMethod = CloudAgeVerificationServiceGrpc.getCloudAgeMethod) == null) {
          CloudAgeVerificationServiceGrpc.getCloudAgeMethod = getCloudAgeMethod =
              io.grpc.MethodDescriptor.<com.senbax.senvend.proto.cloud.v1.CloudAgeRequest, com.senbax.senvend.proto.cloud.v1.CloudAgeResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CloudAge"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.cloud.v1.CloudAgeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.cloud.v1.CloudAgeResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CloudAgeVerificationServiceMethodDescriptorSupplier("CloudAge"))
              .build();
        }
      }
    }
    return getCloudAgeMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CloudAgeVerificationServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceStub>() {
        @java.lang.Override
        public CloudAgeVerificationServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudAgeVerificationServiceStub(channel, callOptions);
        }
      };
    return CloudAgeVerificationServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CloudAgeVerificationServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceBlockingV2Stub>() {
        @java.lang.Override
        public CloudAgeVerificationServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudAgeVerificationServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return CloudAgeVerificationServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CloudAgeVerificationServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceBlockingStub>() {
        @java.lang.Override
        public CloudAgeVerificationServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudAgeVerificationServiceBlockingStub(channel, callOptions);
        }
      };
    return CloudAgeVerificationServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CloudAgeVerificationServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudAgeVerificationServiceFutureStub>() {
        @java.lang.Override
        public CloudAgeVerificationServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudAgeVerificationServiceFutureStub(channel, callOptions);
        }
      };
    return CloudAgeVerificationServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * This service provides the necessary functionality to handle age verification via the SENVEND Terminal.
   * </pre>
   */
  public interface AsyncService {

    /**
     * <pre>
     * Initiates an age verification process on the SENVEND terminal.
     * </pre>
     */
    default io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudAgeRequest> cloudAge(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudAgeResponse> responseObserver) {
      return io.grpc.stub.ServerCalls.asyncUnimplementedStreamingCall(getCloudAgeMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service CloudAgeVerificationService.
   * <pre>
   * This service provides the necessary functionality to handle age verification via the SENVEND Terminal.
   * </pre>
   */
  public static abstract class CloudAgeVerificationServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CloudAgeVerificationServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service CloudAgeVerificationService.
   * <pre>
   * This service provides the necessary functionality to handle age verification via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudAgeVerificationServiceStub
      extends io.grpc.stub.AbstractAsyncStub<CloudAgeVerificationServiceStub> {
    private CloudAgeVerificationServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudAgeVerificationServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudAgeVerificationServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     * Initiates an age verification process on the SENVEND terminal.
     * </pre>
     */
    public io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudAgeRequest> cloudAge(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudAgeResponse> responseObserver) {
      return io.grpc.stub.ClientCalls.asyncBidiStreamingCall(
          getChannel().newCall(getCloudAgeMethod(), getCallOptions()), responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service CloudAgeVerificationService.
   * <pre>
   * This service provides the necessary functionality to handle age verification via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudAgeVerificationServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CloudAgeVerificationServiceBlockingV2Stub> {
    private CloudAgeVerificationServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudAgeVerificationServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudAgeVerificationServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     * <pre>
     * Initiates an age verification process on the SENVEND terminal.
     * </pre>
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<com.senbax.senvend.proto.cloud.v1.CloudAgeRequest, com.senbax.senvend.proto.cloud.v1.CloudAgeResponse>
        cloudAge() {
      return io.grpc.stub.ClientCalls.blockingBidiStreamingCall(
          getChannel(), getCloudAgeMethod(), getCallOptions());
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service CloudAgeVerificationService.
   * <pre>
   * This service provides the necessary functionality to handle age verification via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudAgeVerificationServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CloudAgeVerificationServiceBlockingStub> {
    private CloudAgeVerificationServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudAgeVerificationServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudAgeVerificationServiceBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service CloudAgeVerificationService.
   * <pre>
   * This service provides the necessary functionality to handle age verification via the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudAgeVerificationServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<CloudAgeVerificationServiceFutureStub> {
    private CloudAgeVerificationServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudAgeVerificationServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudAgeVerificationServiceFutureStub(channel, callOptions);
    }
  }

  private static final int METHODID_CLOUD_AGE = 0;

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
        case METHODID_CLOUD_AGE:
          return (io.grpc.stub.StreamObserver<Req>) serviceImpl.cloudAge(
              (io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudAgeResponse>) responseObserver);
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getCloudAgeMethod(),
          io.grpc.stub.ServerCalls.asyncBidiStreamingCall(
            new MethodHandlers<
              com.senbax.senvend.proto.cloud.v1.CloudAgeRequest,
              com.senbax.senvend.proto.cloud.v1.CloudAgeResponse>(
                service, METHODID_CLOUD_AGE)))
        .build();
  }

  private static abstract class CloudAgeVerificationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CloudAgeVerificationServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.senbax.senvend.proto.cloud.v1.CloudProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("CloudAgeVerificationService");
    }
  }

  private static final class CloudAgeVerificationServiceFileDescriptorSupplier
      extends CloudAgeVerificationServiceBaseDescriptorSupplier {
    CloudAgeVerificationServiceFileDescriptorSupplier() {}
  }

  private static final class CloudAgeVerificationServiceMethodDescriptorSupplier
      extends CloudAgeVerificationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CloudAgeVerificationServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CloudAgeVerificationServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CloudAgeVerificationServiceFileDescriptorSupplier())
              .addMethod(getCloudAgeMethod())
              .build();
        }
      }
    }
    return result;
  }
}
