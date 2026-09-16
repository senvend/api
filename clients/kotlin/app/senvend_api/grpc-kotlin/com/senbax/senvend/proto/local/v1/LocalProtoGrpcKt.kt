package com.senbax.senvend.proto.local.v1

import com.senbax.senvend.proto.api.v1.AgeRequest
import com.senbax.senvend.proto.api.v1.AgeResponse
import com.senbax.senvend.proto.api.v1.PayRequest
import com.senbax.senvend.proto.api.v1.PayResponse
import com.senbax.senvend.proto.api.v1.VendRequest
import com.senbax.senvend.proto.api.v1.VendResponse
import com.senbax.senvend.proto.api.v1.VersionRequest
import com.senbax.senvend.proto.api.v1.VersionResponse
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
import com.senbax.senvend.proto.local.v1.AgeVerificationServiceGrpc.getServiceDescriptor as ageVerificationServiceGrpcGetServiceDescriptor
import com.senbax.senvend.proto.local.v1.PayServiceGrpc.getServiceDescriptor as payServiceGrpcGetServiceDescriptor
import com.senbax.senvend.proto.local.v1.VendServiceGrpc.getServiceDescriptor as vendServiceGrpcGetServiceDescriptor
import com.senbax.senvend.proto.local.v1.VersionServiceGrpc.getServiceDescriptor as versionServiceGrpcGetServiceDescriptor

/**
 * Holder for Kotlin coroutine-based client and server APIs for local.v1.AgeVerificationService.
 */
public object AgeVerificationServiceGrpcKt {
  public const val SERVICE_NAME: String = AgeVerificationServiceGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = ageVerificationServiceGrpcGetServiceDescriptor()

  public val ageMethod: MethodDescriptor<AgeRequest, AgeResponse>
    @JvmStatic
    get() = AgeVerificationServiceGrpc.getAgeMethod()

  /**
   * A stub for issuing RPCs to a(n) local.v1.AgeVerificationService service as suspending coroutines.
   */
  @StubFor(AgeVerificationServiceGrpc::class)
  public class AgeVerificationServiceCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<AgeVerificationServiceCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): AgeVerificationServiceCoroutineStub = AgeVerificationServiceCoroutineStub(channel, callOptions)

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
    public fun age(requests: Flow<AgeRequest>, headers: Metadata = Metadata()): Flow<AgeResponse> = bidiStreamingRpc(
      channel,
      AgeVerificationServiceGrpc.getAgeMethod(),
      requests,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the local.v1.AgeVerificationService service based on Kotlin coroutines.
   */
  public abstract class AgeVerificationServiceCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns a [Flow] of responses to an RPC for local.v1.AgeVerificationService.Age.
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
    public open fun age(requests: Flow<AgeRequest>): Flow<AgeResponse> = throw StatusException(UNIMPLEMENTED.withDescription("Method local.v1.AgeVerificationService.Age is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(ageVerificationServiceGrpcGetServiceDescriptor())
      .addMethod(bidiStreamingServerMethodDefinition(
      context = this.context,
      descriptor = AgeVerificationServiceGrpc.getAgeMethod(),
      implementation = ::age
    )).build()
  }
}

/**
 * Holder for Kotlin coroutine-based client and server APIs for local.v1.PayService.
 */
public object PayServiceGrpcKt {
  public const val SERVICE_NAME: String = PayServiceGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = payServiceGrpcGetServiceDescriptor()

  public val payMethod: MethodDescriptor<PayRequest, PayResponse>
    @JvmStatic
    get() = PayServiceGrpc.getPayMethod()

  /**
   * A stub for issuing RPCs to a(n) local.v1.PayService service as suspending coroutines.
   */
  @StubFor(PayServiceGrpc::class)
  public class PayServiceCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<PayServiceCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): PayServiceCoroutineStub = PayServiceCoroutineStub(channel, callOptions)

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
    public fun pay(requests: Flow<PayRequest>, headers: Metadata = Metadata()): Flow<PayResponse> = bidiStreamingRpc(
      channel,
      PayServiceGrpc.getPayMethod(),
      requests,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the local.v1.PayService service based on Kotlin coroutines.
   */
  public abstract class PayServiceCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns a [Flow] of responses to an RPC for local.v1.PayService.Pay.
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
    public open fun pay(requests: Flow<PayRequest>): Flow<PayResponse> = throw StatusException(UNIMPLEMENTED.withDescription("Method local.v1.PayService.Pay is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(payServiceGrpcGetServiceDescriptor())
      .addMethod(bidiStreamingServerMethodDefinition(
      context = this.context,
      descriptor = PayServiceGrpc.getPayMethod(),
      implementation = ::pay
    )).build()
  }
}

/**
 * Holder for Kotlin coroutine-based client and server APIs for local.v1.VendService.
 */
public object VendServiceGrpcKt {
  public const val SERVICE_NAME: String = VendServiceGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = vendServiceGrpcGetServiceDescriptor()

  public val vendMethod: MethodDescriptor<VendRequest, VendResponse>
    @JvmStatic
    get() = VendServiceGrpc.getVendMethod()

  /**
   * A stub for issuing RPCs to a(n) local.v1.VendService service as suspending coroutines.
   */
  @StubFor(VendServiceGrpc::class)
  public class VendServiceCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<VendServiceCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): VendServiceCoroutineStub = VendServiceCoroutineStub(channel, callOptions)

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
    public fun vend(requests: Flow<VendRequest>, headers: Metadata = Metadata()): Flow<VendResponse> = bidiStreamingRpc(
      channel,
      VendServiceGrpc.getVendMethod(),
      requests,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the local.v1.VendService service based on Kotlin coroutines.
   */
  public abstract class VendServiceCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns a [Flow] of responses to an RPC for local.v1.VendService.Vend.
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
    public open fun vend(requests: Flow<VendRequest>): Flow<VendResponse> = throw StatusException(UNIMPLEMENTED.withDescription("Method local.v1.VendService.Vend is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(vendServiceGrpcGetServiceDescriptor())
      .addMethod(bidiStreamingServerMethodDefinition(
      context = this.context,
      descriptor = VendServiceGrpc.getVendMethod(),
      implementation = ::vend
    )).build()
  }
}

/**
 * Holder for Kotlin coroutine-based client and server APIs for local.v1.VersionService.
 */
public object VersionServiceGrpcKt {
  public const val SERVICE_NAME: String = VersionServiceGrpc.SERVICE_NAME

  @JvmStatic
  public val serviceDescriptor: ServiceDescriptor
    get() = versionServiceGrpcGetServiceDescriptor()

  public val versionMethod: MethodDescriptor<VersionRequest, VersionResponse>
    @JvmStatic
    get() = VersionServiceGrpc.getVersionMethod()

  /**
   * A stub for issuing RPCs to a(n) local.v1.VersionService service as suspending coroutines.
   */
  @StubFor(VersionServiceGrpc::class)
  public class VersionServiceCoroutineStub @JvmOverloads constructor(
    channel: Channel,
    callOptions: CallOptions = DEFAULT,
  ) : AbstractCoroutineStub<VersionServiceCoroutineStub>(channel, callOptions) {
    override fun build(channel: Channel, callOptions: CallOptions): VersionServiceCoroutineStub = VersionServiceCoroutineStub(channel, callOptions)

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
    public suspend fun version(request: VersionRequest, headers: Metadata = Metadata()): VersionResponse = unaryRpc(
      channel,
      VersionServiceGrpc.getVersionMethod(),
      request,
      callOptions,
      headers
    )
  }

  /**
   * Skeletal implementation of the local.v1.VersionService service based on Kotlin coroutines.
   */
  public abstract class VersionServiceCoroutineImplBase(
    coroutineContext: CoroutineContext = EmptyCoroutineContext,
  ) : AbstractCoroutineServerImpl(coroutineContext) {
    /**
     * Returns the response to an RPC for local.v1.VersionService.Version.
     *
     * If this method fails with a [StatusException], the RPC will fail with the corresponding
     * [io.grpc.Status].  If this method fails with a [java.util.concurrent.CancellationException], the RPC will fail
     * with status `Status.CANCELLED`.  If this method fails for any other reason, the RPC will
     * fail with `Status.UNKNOWN` with the exception as a cause.
     *
     * @param request The request from the client.
     */
    public open suspend fun version(request: VersionRequest): VersionResponse = throw StatusException(UNIMPLEMENTED.withDescription("Method local.v1.VersionService.Version is unimplemented"))

    final override fun bindService(): ServerServiceDefinition = builder(versionServiceGrpcGetServiceDescriptor())
      .addMethod(unaryServerMethodDefinition(
      context = this.context,
      descriptor = VersionServiceGrpc.getVersionMethod(),
      implementation = ::version
    )).build()
  }
}
