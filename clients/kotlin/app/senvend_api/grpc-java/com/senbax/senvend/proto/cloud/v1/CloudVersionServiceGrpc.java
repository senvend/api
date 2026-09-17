package com.senbax.senvend.proto.cloud.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * This service provides version information for the software on the SENVEND Terminal.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CloudVersionServiceGrpc {

  private CloudVersionServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "cloud.v1.CloudVersionService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudVersionRequest,
      com.senbax.senvend.proto.cloud.v1.CloudVersionResponse> getCloudVersionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CloudVersion",
      requestType = com.senbax.senvend.proto.cloud.v1.CloudVersionRequest.class,
      responseType = com.senbax.senvend.proto.cloud.v1.CloudVersionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudVersionRequest,
      com.senbax.senvend.proto.cloud.v1.CloudVersionResponse> getCloudVersionMethod() {
    io.grpc.MethodDescriptor<com.senbax.senvend.proto.cloud.v1.CloudVersionRequest, com.senbax.senvend.proto.cloud.v1.CloudVersionResponse> getCloudVersionMethod;
    if ((getCloudVersionMethod = CloudVersionServiceGrpc.getCloudVersionMethod) == null) {
      synchronized (CloudVersionServiceGrpc.class) {
        if ((getCloudVersionMethod = CloudVersionServiceGrpc.getCloudVersionMethod) == null) {
          CloudVersionServiceGrpc.getCloudVersionMethod = getCloudVersionMethod =
              io.grpc.MethodDescriptor.<com.senbax.senvend.proto.cloud.v1.CloudVersionRequest, com.senbax.senvend.proto.cloud.v1.CloudVersionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CloudVersion"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.cloud.v1.CloudVersionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.cloud.v1.CloudVersionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CloudVersionServiceMethodDescriptorSupplier("CloudVersion"))
              .build();
        }
      }
    }
    return getCloudVersionMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CloudVersionServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceStub>() {
        @java.lang.Override
        public CloudVersionServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudVersionServiceStub(channel, callOptions);
        }
      };
    return CloudVersionServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CloudVersionServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceBlockingV2Stub>() {
        @java.lang.Override
        public CloudVersionServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudVersionServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return CloudVersionServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CloudVersionServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceBlockingStub>() {
        @java.lang.Override
        public CloudVersionServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudVersionServiceBlockingStub(channel, callOptions);
        }
      };
    return CloudVersionServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CloudVersionServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CloudVersionServiceFutureStub>() {
        @java.lang.Override
        public CloudVersionServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CloudVersionServiceFutureStub(channel, callOptions);
        }
      };
    return CloudVersionServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public interface AsyncService {

    /**
     * <pre>
     * Returns the version information of the software and API on the SENVEND Terminal.
     * </pre>
     */
    default void cloudVersion(com.senbax.senvend.proto.cloud.v1.CloudVersionRequest request,
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudVersionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCloudVersionMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service CloudVersionService.
   * <pre>
   * This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static abstract class CloudVersionServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CloudVersionServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service CloudVersionService.
   * <pre>
   * This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudVersionServiceStub
      extends io.grpc.stub.AbstractAsyncStub<CloudVersionServiceStub> {
    private CloudVersionServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudVersionServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudVersionServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     * Returns the version information of the software and API on the SENVEND Terminal.
     * </pre>
     */
    public void cloudVersion(com.senbax.senvend.proto.cloud.v1.CloudVersionRequest request,
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudVersionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCloudVersionMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service CloudVersionService.
   * <pre>
   * This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudVersionServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CloudVersionServiceBlockingV2Stub> {
    private CloudVersionServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudVersionServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudVersionServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     * <pre>
     * Returns the version information of the software and API on the SENVEND Terminal.
     * </pre>
     */
    public com.senbax.senvend.proto.cloud.v1.CloudVersionResponse cloudVersion(com.senbax.senvend.proto.cloud.v1.CloudVersionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCloudVersionMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service CloudVersionService.
   * <pre>
   * This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudVersionServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CloudVersionServiceBlockingStub> {
    private CloudVersionServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudVersionServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudVersionServiceBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     * Returns the version information of the software and API on the SENVEND Terminal.
     * </pre>
     */
    public com.senbax.senvend.proto.cloud.v1.CloudVersionResponse cloudVersion(com.senbax.senvend.proto.cloud.v1.CloudVersionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCloudVersionMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service CloudVersionService.
   * <pre>
   * This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class CloudVersionServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<CloudVersionServiceFutureStub> {
    private CloudVersionServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CloudVersionServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CloudVersionServiceFutureStub(channel, callOptions);
    }

    /**
     * <pre>
     * Returns the version information of the software and API on the SENVEND Terminal.
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.senbax.senvend.proto.cloud.v1.CloudVersionResponse> cloudVersion(
        com.senbax.senvend.proto.cloud.v1.CloudVersionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCloudVersionMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CLOUD_VERSION = 0;

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
        case METHODID_CLOUD_VERSION:
          serviceImpl.cloudVersion((com.senbax.senvend.proto.cloud.v1.CloudVersionRequest) request,
              (io.grpc.stub.StreamObserver<com.senbax.senvend.proto.cloud.v1.CloudVersionResponse>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getCloudVersionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.senbax.senvend.proto.cloud.v1.CloudVersionRequest,
              com.senbax.senvend.proto.cloud.v1.CloudVersionResponse>(
                service, METHODID_CLOUD_VERSION)))
        .build();
  }

  private static abstract class CloudVersionServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CloudVersionServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.senbax.senvend.proto.cloud.v1.CloudProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("CloudVersionService");
    }
  }

  private static final class CloudVersionServiceFileDescriptorSupplier
      extends CloudVersionServiceBaseDescriptorSupplier {
    CloudVersionServiceFileDescriptorSupplier() {}
  }

  private static final class CloudVersionServiceMethodDescriptorSupplier
      extends CloudVersionServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CloudVersionServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CloudVersionServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CloudVersionServiceFileDescriptorSupplier())
              .addMethod(getCloudVersionMethod())
              .build();
        }
      }
    }
    return result;
  }
}
