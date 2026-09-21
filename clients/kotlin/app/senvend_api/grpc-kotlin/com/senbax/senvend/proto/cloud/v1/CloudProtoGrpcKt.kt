package com.senbax.senvend.proto.cloud.v1

import io.grpc.CallOptions
import io.grpc.CallOptions.DEFAULT
import io.grpc.Channel
import io.grpc.Metadata
import io.grpc.MethodDescriptor
import io.grpc.ServerServiceDefinition
import io.grpc.ServerServiceDefinition.builder
import io.grpc.ServiceDescriptor
import io.grpc.Status.UNIMPLEMENTED
import io.grpc.StatusException
import io.grpc.kotlin.AbstractCoroutineServerImpl
import io.grpc.kotlin.AbstractCoroutineStub
import io.grpc.kotlin.ClientCalls.bidiStreamingRpc
import io.grpc.kotlin.ClientCalls.unaryRpc
import io.grpc.kotlin.ServerCalls.bidiStreamingServerMethodDefinition
import io.grpc.kotlin.ServerCalls.unaryServerMethodDefinition
import io.grpc.kotlin.StubFor
import kotlin.String
import kotlin.coroutines.CoroutineContext
import kotlin.coroutines.EmptyCoroutineContext
import kotlin.jvm.JvmOverloads
import kotlin.jvm.JvmStatic
import kotlinx.coroutines.flow.Flow
import com.senbax.senvend.proto.cloud.v1.CloudAgeVerificationServiceGrpc.getServiceDescriptor as cloudAgeVerificationServiceGrpcGetServiceDescriptor
import com.senbax.senvend.proto.cloud.v1.CloudPayServiceGrpc.getServiceDescriptor as cloudPayServiceGrpcGetServiceDescriptor
import com.senbax.senvend.proto.cloud.v1.CloudVersionServiceGrpc.getServiceDescriptor as cloudVersionServiceGrpcGetServiceDescriptor

/**
 * Holder for Kotlin coroutine-based client and server APIs for cloud.v1.CloudAgeVerificationService.
 */
public object CloudAgeVerificationServiceGrpcKt {
  public const val SERVICE_NAME: String = CloudAgeVerificationServiceGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = cloudAgeVerificationServiceGrpcGetServiceDescriptor()

  public val cloudAgeMethod: MethodDescriptor<CloudAgeRequest, CloudAgeResponse>
    @JvmStatic
    get() = CloudAgeVerificationServiceGrpc.getCloudAgeMethod()

  /**
   * A stub for issuing RPCs to a(n) cloud.v1.CloudAgeVerificationService service as suspending coroutines.
   */
  @StubFor(CloudAgeVerificationServiceGrpc::class)
  public class CloudAgeVerificationServiceCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<CloudAgeVerificationServiceCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): CloudAgeVerificationServiceCoroutineStub = CloudAgeVerificationServiceCoroutineStub(channel, callOptions)

    /**
     * Returns a [Flow] that, when collected, executes this RPC and emits responses from the
     * server as they arrive.  That flow finishes normally if the server closes its response with
     * [`Status.OK`][io.grpc.Status], and fails by throwing a [StatusException] otherwise.  If
     * collecting the flow downstream fails exceptionally (including via cancellation), the RPC
     * is cancelled with that exception as a cause.
     *
     * The [Flow] of requests is collected once each time the [Flow] of responses is
     * collected. If collection of the [Flow] of responses completes normally or
     * exceptionally before collection of `requests` completes, the collection of
     * `requests` is cancelled.  If the collection of `requests` completes
     * exceptionally for any other reason, then the collection of the [Flow] of responses
     * completes exceptionally for the same reason and the RPC is cancelled with that reason.
     *
     * @param requests A [Flow] of request messages.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return A flow that, when collected, emits the responses from the server.
     */
    public fun cloudAge(requests: Flow<CloudAgeRequest>, headers: Metadata = Metadata()): Flow<CloudAgeResponse> = bidiStreamingRpc(
      channel,
      CloudAgeVerificationServiceGrpc.getCloudAgeMethod(),
      requests,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the cloud.v1.CloudAgeVerificationService service based on Kotlin coroutines.
   */
  public abstract class CloudAgeVerificationServiceCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns a [Flow] of responses to an RPC for cloud.v1.CloudAgeVerificationService.CloudAge.
     *
     * If creating or collecting the returned flow fails with a [StatusException], the RPC
     * will fail with the corresponding [io.grpc.Status].  If it fails with a
     * [java.util.concurrent.CancellationException], the RPC will fail with status `Status.CANCELLED`.  If creating
     * or collecting the returned flow fails for any other reason, the RPC will fail with
     * `Status.UNKNOWN` with the exception as a cause.
     *
     * @param requests A [Flow] of requests from the client.  This flow can be
     *        collected only once and throws [java.lang.IllegalStateException] on attempts to collect
     *        it more than once.
     */
    public open fun cloudAge(requests: Flow<CloudAgeRequest>): Flow<CloudAgeResponse> = throw StatusException(UNIMPLEMENTED.withDescription("Method cloud.v1.CloudAgeVerificationService.CloudAge is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(cloudAgeVerificationServiceGrpcGetServiceDescriptor())
      .addMethod(bidiStreamingServerMethodDefinition(
      context = this.context,
      descriptor = CloudAgeVerificationServiceGrpc.getCloudAgeMethod(),
      implementation = ::cloudAge
    )).build()
  }
}

/**
 * Holder for Kotlin coroutine-based client and server APIs for cloud.v1.CloudPayService.
 */
public object CloudPayServiceGrpcKt {
  public const val SERVICE_NAME: String = CloudPayServiceGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = cloudPayServiceGrpcGetServiceDescriptor()

  public val cloudPayMethod: MethodDescriptor<CloudPayRequest, CloudPayResponse>
    @JvmStatic
    get() = CloudPayServiceGrpc.getCloudPayMethod()

  /**
   * A stub for issuing RPCs to a(n) cloud.v1.CloudPayService service as suspending coroutines.
   */
  @StubFor(CloudPayServiceGrpc::class)
  public class CloudPayServiceCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<CloudPayServiceCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): CloudPayServiceCoroutineStub = CloudPayServiceCoroutineStub(channel, callOptions)

    /**
     * Returns a [Flow] that, when collected, executes this RPC and emits responses from the
     * server as they arrive.  That flow finishes normally if the server closes its response with
     * [`Status.OK`][io.grpc.Status], and fails by throwing a [StatusException] otherwise.  If
     * collecting the flow downstream fails exceptionally (including via cancellation), the RPC
     * is cancelled with that exception as a cause.
     *
     * The [Flow] of requests is collected once each time the [Flow] of responses is
     * collected. If collection of the [Flow] of responses completes normally or
     * exceptionally before collection of `requests` completes, the collection of
     * `requests` is cancelled.  If the collection of `requests` completes
     * exceptionally for any other reason, then the collection of the [Flow] of responses
     * completes exceptionally for the same reason and the RPC is cancelled with that reason.
     *
     * @param requests A [Flow] of request messages.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return A flow that, when collected, emits the responses from the server.
     */
    public fun cloudPay(requests: Flow<CloudPayRequest>, headers: Metadata = Metadata()): Flow<CloudPayResponse> = bidiStreamingRpc(
      channel,
      CloudPayServiceGrpc.getCloudPayMethod(),
      requests,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the cloud.v1.CloudPayService service based on Kotlin coroutines.
   */
  public abstract class CloudPayServiceCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns a [Flow] of responses to an RPC for cloud.v1.CloudPayService.CloudPay.
     *
     * If creating or collecting the returned flow fails with a [StatusException], the RPC
     * will fail with the corresponding [io.grpc.Status].  If it fails with a
     * [java.util.concurrent.CancellationException], the RPC will fail with status `Status.CANCELLED`.  If creating
     * or collecting the returned flow fails for any other reason, the RPC will fail with
     * `Status.UNKNOWN` with the exception as a cause.
     *
     * @param requests A [Flow] of requests from the client.  This flow can be
     *        collected only once and throws [java.lang.IllegalStateException] on attempts to collect
     *        it more than once.
     */
    public open fun cloudPay(requests: Flow<CloudPayRequest>): Flow<CloudPayResponse> = throw StatusException(UNIMPLEMENTED.withDescription("Method cloud.v1.CloudPayService.CloudPay is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(cloudPayServiceGrpcGetServiceDescriptor())
      .addMethod(bidiStreamingServerMethodDefinition(
      context = this.context,
      descriptor = CloudPayServiceGrpc.getCloudPayMethod(),
      implementation = ::cloudPay
    )).build()
  }
}

/**
 * Holder for Kotlin coroutine-based client and server APIs for cloud.v1.CloudVersionService.
 */
public object CloudVersionServiceGrpcKt {
  public const val SERVICE_NAME: String = CloudVersionServiceGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = cloudVersionServiceGrpcGetServiceDescriptor()

  public val cloudVersionMethod: MethodDescriptor<CloudVersionRequest, CloudVersionResponse>
    @JvmStatic
    get() = CloudVersionServiceGrpc.getCloudVersionMethod()

  /**
   * A stub for issuing RPCs to a(n) cloud.v1.CloudVersionService service as suspending coroutines.
   */
  @StubFor(CloudVersionServiceGrpc::class)
  public class CloudVersionServiceCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<CloudVersionServiceCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): CloudVersionServiceCoroutineStub = CloudVersionServiceCoroutineStub(channel, callOptions)

    /**
     * Executes this RPC and returns the response message, suspending until the RPC completes
     * with [`Status.OK`][io.grpc.Status].  If the RPC completes with another status, a corresponding
     * [StatusException] is thrown.  If this coroutine is cancelled, the RPC is also cancelled
     * with the corresponding exception as a cause.
     *
     * @param request The request message to send to the server.
     *
     * @param headers Metadata to attach to the request.  Most users will not need this.
     *
     * @return The single response from the server.
     */
    public suspend fun cloudVersion(request: CloudVersionRequest, headers: Metadata = Metadata()): CloudVersionResponse = unaryRpc(
      channel,
      CloudVersionServiceGrpc.getCloudVersionMethod(),
      request,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the cloud.v1.CloudVersionService service based on Kotlin coroutines.
   */
  public abstract class CloudVersionServiceCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns the response to an RPC for cloud.v1.CloudVersionService.CloudVersion.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException], the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun cloudVersion(request: CloudVersionRequest): CloudVersionResponse = throw StatusException(UNIMPLEMENTED.withDescription("Method cloud.v1.CloudVersionService.CloudVersion is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(cloudVersionServiceGrpcGetServiceDescriptor())
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = CloudVersionServiceGrpc.getCloudVersionMethod(),
      implementation = ::cloudVersion
    )).build()
  }
}
