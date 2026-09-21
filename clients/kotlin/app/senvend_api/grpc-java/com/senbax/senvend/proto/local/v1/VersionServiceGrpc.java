package com.senbax.senvend.proto.local.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 *This service provides version information for the software on the SENVEND Terminal.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class VersionServiceGrpc {

  private VersionServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "local.v1.VersionService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.VersionRequest,
      com.senbax.senvend.proto.api.v1.VersionResponse> getVersionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Version",
      requestType = com.senbax.senvend.proto.api.v1.VersionRequest.class,
      responseType = com.senbax.senvend.proto.api.v1.VersionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.VersionRequest,
      com.senbax.senvend.proto.api.v1.VersionResponse> getVersionMethod() {
    io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.VersionRequest, com.senbax.senvend.proto.api.v1.VersionResponse> getVersionMethod;
    if ((getVersionMethod = VersionServiceGrpc.getVersionMethod) == null) {
      synchronized (VersionServiceGrpc.class) {
        if ((getVersionMethod = VersionServiceGrpc.getVersionMethod) == null) {
          VersionServiceGrpc.getVersionMethod = getVersionMethod =
              io.grpc.MethodDescriptor.<com.senbax.senvend.proto.api.v1.VersionRequest, com.senbax.senvend.proto.api.v1.VersionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Version"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.VersionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.VersionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new VersionServiceMethodDescriptorSupplier("Version"))
              .build();
        }
      }
    }
    return getVersionMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static VersionServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VersionServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VersionServiceStub>() {
        @java.lang.Override
        public VersionServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VersionServiceStub(channel, callOptions);
        }
      };
    return VersionServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static VersionServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VersionServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VersionServiceBlockingV2Stub>() {
        @java.lang.Override
        public VersionServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VersionServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return VersionServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static VersionServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VersionServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VersionServiceBlockingStub>() {
        @java.lang.Override
        public VersionServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VersionServiceBlockingStub(channel, callOptions);
        }
      };
    return VersionServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static VersionServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VersionServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VersionServiceFutureStub>() {
        @java.lang.Override
        public VersionServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VersionServiceFutureStub(channel, callOptions);
        }
      };
    return VersionServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   *This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public interface AsyncService {

    /**
     * <pre>
     *Returns the version information of the software and API on the SENVEND Terminal.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given a new UUID is generated per request.&#92;
     *These are mostly provided for the cloud API functionality.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Standard version request
     *&gt; **-&gt;** {}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "appVersion": {"major": 1, "minor": 3, "patch": 11}, "apiVersion": {"major": 1}}
     *&lt;/details&gt;
     * </pre>
     */
    default void version(com.senbax.senvend.proto.api.v1.VersionRequest request,
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VersionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getVersionMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service VersionService.
   * <pre>
   *This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static abstract class VersionServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return VersionServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service VersionService.
   * <pre>
   *This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class VersionServiceStub
      extends io.grpc.stub.AbstractAsyncStub<VersionServiceStub> {
    private VersionServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VersionServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VersionServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     *Returns the version information of the software and API on the SENVEND Terminal.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given a new UUID is generated per request.&#92;
     *These are mostly provided for the cloud API functionality.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Standard version request
     *&gt; **-&gt;** {}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "appVersion": {"major": 1, "minor": 3, "patch": 11}, "apiVersion": {"major": 1}}
     *&lt;/details&gt;
     * </pre>
     */
    public void version(com.senbax.senvend.proto.api.v1.VersionRequest request,
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VersionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getVersionMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service VersionService.
   * <pre>
   *This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class VersionServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<VersionServiceBlockingV2Stub> {
    private VersionServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VersionServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VersionServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     * <pre>
     *Returns the version information of the software and API on the SENVEND Terminal.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given a new UUID is generated per request.&#92;
     *These are mostly provided for the cloud API functionality.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Standard version request
     *&gt; **-&gt;** {}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "appVersion": {"major": 1, "minor": 3, "patch": 11}, "apiVersion": {"major": 1}}
     *&lt;/details&gt;
     * </pre>
     */
    public com.senbax.senvend.proto.api.v1.VersionResponse version(com.senbax.senvend.proto.api.v1.VersionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getVersionMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service VersionService.
   * <pre>
   *This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class VersionServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<VersionServiceBlockingStub> {
    private VersionServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VersionServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VersionServiceBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     *Returns the version information of the software and API on the SENVEND Terminal.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given a new UUID is generated per request.&#92;
     *These are mostly provided for the cloud API functionality.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Standard version request
     *&gt; **-&gt;** {}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "appVersion": {"major": 1, "minor": 3, "patch": 11}, "apiVersion": {"major": 1}}
     *&lt;/details&gt;
     * </pre>
     */
    public com.senbax.senvend.proto.api.v1.VersionResponse version(com.senbax.senvend.proto.api.v1.VersionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getVersionMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service VersionService.
   * <pre>
   *This service provides version information for the software on the SENVEND Terminal.
   * </pre>
   */
  public static final class VersionServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<VersionServiceFutureStub> {
    private VersionServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VersionServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VersionServiceFutureStub(channel, callOptions);
    }

    /**
     * <pre>
     *Returns the version information of the software and API on the SENVEND Terminal.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given a new UUID is generated per request.&#92;
     *These are mostly provided for the cloud API functionality.
     *&lt;/details&gt;
     *&lt;details&gt;
     *&lt;summary&gt;Examples&lt;/summary&gt;
     *###### Standard version request
     *&gt; **-&gt;** {}
     *&gt; **&#92;&lt;-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "appVersion": {"major": 1, "minor": 3, "patch": 11}, "apiVersion": {"major": 1}}
     *&lt;/details&gt;
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.senbax.senvend.proto.api.v1.VersionResponse> version(
        com.senbax.senvend.proto.api.v1.VersionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getVersionMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_VERSION = 0;

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
        case METHODID_VERSION:
          serviceImpl.version((com.senbax.senvend.proto.api.v1.VersionRequest) request,
              (io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VersionResponse>) responseObserver);
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
          getVersionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.senbax.senvend.proto.api.v1.VersionRequest,
              com.senbax.senvend.proto.api.v1.VersionResponse>(
                service, METHODID_VERSION)))
        .build();
  }

  private static abstract class VersionServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    VersionServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.senbax.senvend.proto.local.v1.LocalProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("VersionService");
    }
  }

  private static final class VersionServiceFileDescriptorSupplier
      extends VersionServiceBaseDescriptorSupplier {
    VersionServiceFileDescriptorSupplier() {}
  }

  private static final class VersionServiceMethodDescriptorSupplier
      extends VersionServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    VersionServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (VersionServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new VersionServiceFileDescriptorSupplier())
              .addMethod(getVersionMethod())
              .build();
        }
      }
    }
    return result;
  }
}
