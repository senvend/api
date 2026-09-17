package com.senbax.senvend.proto.local.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 *This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class VendServiceGrpc {

  private VendServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "local.v1.VendService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.VendRequest,
      com.senbax.senvend.proto.api.v1.VendResponse> getVendMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Vend",
      requestType = com.senbax.senvend.proto.api.v1.VendRequest.class,
      responseType = com.senbax.senvend.proto.api.v1.VendResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
  public static io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.VendRequest,
      com.senbax.senvend.proto.api.v1.VendResponse> getVendMethod() {
    io.grpc.MethodDescriptor<com.senbax.senvend.proto.api.v1.VendRequest, com.senbax.senvend.proto.api.v1.VendResponse> getVendMethod;
    if ((getVendMethod = VendServiceGrpc.getVendMethod) == null) {
      synchronized (VendServiceGrpc.class) {
        if ((getVendMethod = VendServiceGrpc.getVendMethod) == null) {
          VendServiceGrpc.getVendMethod = getVendMethod =
              io.grpc.MethodDescriptor.<com.senbax.senvend.proto.api.v1.VendRequest, com.senbax.senvend.proto.api.v1.VendResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Vend"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.VendRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.senbax.senvend.proto.api.v1.VendResponse.getDefaultInstance()))
              .setSchemaDescriptor(new VendServiceMethodDescriptorSupplier("Vend"))
              .build();
        }
      }
    }
    return getVendMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static VendServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VendServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VendServiceStub>() {
        @java.lang.Override
        public VendServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VendServiceStub(channel, callOptions);
        }
      };
    return VendServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static VendServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VendServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VendServiceBlockingV2Stub>() {
        @java.lang.Override
        public VendServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VendServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return VendServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static VendServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VendServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VendServiceBlockingStub>() {
        @java.lang.Override
        public VendServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VendServiceBlockingStub(channel, callOptions);
        }
      };
    return VendServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static VendServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<VendServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<VendServiceFutureStub>() {
        @java.lang.Override
        public VendServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new VendServiceFutureStub(channel, callOptions);
        }
      };
    return VendServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   *This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
   * </pre>
   */
  public interface AsyncService {

    /**
     * <pre>
     *Initiates a vending process on a machine connected to the SENVEND Terminal.&#92;
     *Accepts a stream of VendRequest for starting and controlling vending.&#92;
     *Returns a stream of VendResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a vending process, after a reconnect the currently running process can still be controlled.&#92;
     *VendResponses that occurred during the disconnect are lost though.
     *- The minimum quantity to vend has to be 1, otherwise the request will be rejected.
     *- Items are vended via LineItem messages. The `price` field is optional and not necessary for vending.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- There can only be one vending process at a time. Multiple items can either be vended one-by-one,&#92;
     *or by combining them all into one VendStart message.
     *- For each individual vending attempt, a VendEvent is sent back, indicating success or failure.
     *- For LineItem entries with a quantity greater than 1, items will be vended one-by-one until all are successful, or the FIRST vending failure.&#92;
     *The resulting VendEvent failure message will also contain the number of successfully vended items.
     *- If multiple LineItem entries are given, the list is vended according to the order of the entries in the message,
     *regardless of success or failure.
     *- Vending via this endpoint is also available when there is an ongoing payment,
     *specifically after a payment was APPROVED but before GOODS_ISSUED.&#92;
     *If you don't need the UUIDs of this endpoint, consider using the VendStart messages of the `Pay` endpoint.
     *- Unlike the VendStart messages embedded within PayRequest, this API provides UUIDs per VendRequest.&#92;
     *If you require precise control over the vending process, use this API to vend single items,
     *and match requests and answers via their UUIDs.
     *- The cancel request is provided to enable stopping midway during vending of a list of LineItem entries.&#92;
     *If vending a single item, a cancel usually arrives too late to stop the process.
     *&lt;/details&gt;
     * </pre>
     */
    default io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VendRequest> vend(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VendResponse> responseObserver) {
      return io.grpc.stub.ServerCalls.asyncUnimplementedStreamingCall(getVendMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service VendService.
   * <pre>
   *This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
   * </pre>
   */
  public static abstract class VendServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return VendServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service VendService.
   * <pre>
   *This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
   * </pre>
   */
  public static final class VendServiceStub
      extends io.grpc.stub.AbstractAsyncStub<VendServiceStub> {
    private VendServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VendServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VendServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     *Initiates a vending process on a machine connected to the SENVEND Terminal.&#92;
     *Accepts a stream of VendRequest for starting and controlling vending.&#92;
     *Returns a stream of VendResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a vending process, after a reconnect the currently running process can still be controlled.&#92;
     *VendResponses that occurred during the disconnect are lost though.
     *- The minimum quantity to vend has to be 1, otherwise the request will be rejected.
     *- Items are vended via LineItem messages. The `price` field is optional and not necessary for vending.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- There can only be one vending process at a time. Multiple items can either be vended one-by-one,&#92;
     *or by combining them all into one VendStart message.
     *- For each individual vending attempt, a VendEvent is sent back, indicating success or failure.
     *- For LineItem entries with a quantity greater than 1, items will be vended one-by-one until all are successful, or the FIRST vending failure.&#92;
     *The resulting VendEvent failure message will also contain the number of successfully vended items.
     *- If multiple LineItem entries are given, the list is vended according to the order of the entries in the message,
     *regardless of success or failure.
     *- Vending via this endpoint is also available when there is an ongoing payment,
     *specifically after a payment was APPROVED but before GOODS_ISSUED.&#92;
     *If you don't need the UUIDs of this endpoint, consider using the VendStart messages of the `Pay` endpoint.
     *- Unlike the VendStart messages embedded within PayRequest, this API provides UUIDs per VendRequest.&#92;
     *If you require precise control over the vending process, use this API to vend single items,
     *and match requests and answers via their UUIDs.
     *- The cancel request is provided to enable stopping midway during vending of a list of LineItem entries.&#92;
     *If vending a single item, a cancel usually arrives too late to stop the process.
     *&lt;/details&gt;
     * </pre>
     */
    public io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VendRequest> vend(
        io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VendResponse> responseObserver) {
      return io.grpc.stub.ClientCalls.asyncBidiStreamingCall(
          getChannel().newCall(getVendMethod(), getCallOptions()), responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service VendService.
   * <pre>
   *This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
   * </pre>
   */
  public static final class VendServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<VendServiceBlockingV2Stub> {
    private VendServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VendServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VendServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     * <pre>
     *Initiates a vending process on a machine connected to the SENVEND Terminal.&#92;
     *Accepts a stream of VendRequest for starting and controlling vending.&#92;
     *Returns a stream of VendResponse containing status and error return messages.
     *&lt;details open&gt;
     *&lt;summary&gt;API Constraints&lt;/summary&gt;
     *- Request ids are optional.&#92;
     *If none is given and a process is running, the request is applied to that running process.&#92;
     *Otherwise a new UUID is generated per request.
     *- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same id.&#92;
     *An empty id will work as well if the original request started the currently running process.
     *- All given ids must be valid version 4 UUIDs.
     *- If disconnected during a vending process, after a reconnect the currently running process can still be controlled.&#92;
     *VendResponses that occurred during the disconnect are lost though.
     *- The minimum quantity to vend has to be 1, otherwise the request will be rejected.
     *- Items are vended via LineItem messages. The `price` field is optional and not necessary for vending.
     *&lt;/details&gt;
     *&lt;details open&gt;
     *&lt;summary&gt;Process Constraints&lt;/summary&gt;
     *- There can only be one vending process at a time. Multiple items can either be vended one-by-one,&#92;
     *or by combining them all into one VendStart message.
     *- For each individual vending attempt, a VendEvent is sent back, indicating success or failure.
     *- For LineItem entries with a quantity greater than 1, items will be vended one-by-one until all are successful, or the FIRST vending failure.&#92;
     *The resulting VendEvent failure message will also contain the number of successfully vended items.
     *- If multiple LineItem entries are given, the list is vended according to the order of the entries in the message,
     *regardless of success or failure.
     *- Vending via this endpoint is also available when there is an ongoing payment,
     *specifically after a payment was APPROVED but before GOODS_ISSUED.&#92;
     *If you don't need the UUIDs of this endpoint, consider using the VendStart messages of the `Pay` endpoint.
     *- Unlike the VendStart messages embedded within PayRequest, this API provides UUIDs per VendRequest.&#92;
     *If you require precise control over the vending process, use this API to vend single items,
     *and match requests and answers via their UUIDs.
     *- The cancel request is provided to enable stopping midway during vending of a list of LineItem entries.&#92;
     *If vending a single item, a cancel usually arrives too late to stop the process.
     *&lt;/details&gt;
     * </pre>
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<com.senbax.senvend.proto.api.v1.VendRequest, com.senbax.senvend.proto.api.v1.VendResponse>
        vend() {
      return io.grpc.stub.ClientCalls.blockingBidiStreamingCall(
          getChannel(), getVendMethod(), getCallOptions());
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service VendService.
   * <pre>
   *This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
   * </pre>
   */
  public static final class VendServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<VendServiceBlockingStub> {
    private VendServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VendServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VendServiceBlockingStub(channel, callOptions);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service VendService.
   * <pre>
   *This service provides the necessary functionality to vend products via a vending machine connected to the SENVEND Terminal.
   * </pre>
   */
  public static final class VendServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<VendServiceFutureStub> {
    private VendServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected VendServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new VendServiceFutureStub(channel, callOptions);
    }
  }

  private static final int METHODID_VEND = 0;

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
        case METHODID_VEND:
          return (io.grpc.stub.StreamObserver<Req>) serviceImpl.vend(
              (io.grpc.stub.StreamObserver<com.senbax.senvend.proto.api.v1.VendResponse>) responseObserver);
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getVendMethod(),
          io.grpc.stub.ServerCalls.asyncBidiStreamingCall(
            new MethodHandlers<
              com.senbax.senvend.proto.api.v1.VendRequest,
              com.senbax.senvend.proto.api.v1.VendResponse>(
                service, METHODID_VEND)))
        .build();
  }

  private static abstract class VendServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    VendServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.senbax.senvend.proto.local.v1.LocalProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("VendService");
    }
  }

  private static final class VendServiceFileDescriptorSupplier
      extends VendServiceBaseDescriptorSupplier {
    VendServiceFileDescriptorSupplier() {}
  }

  private static final class VendServiceMethodDescriptorSupplier
      extends VendServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    VendServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (VendServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new VendServiceFileDescriptorSupplier())
              .addMethod(getVendMethod())
              .build();
        }
      }
    }
    return result;
  }
}
