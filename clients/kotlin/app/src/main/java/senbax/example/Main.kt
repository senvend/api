package senbax.example

import com.senbax.senvend.proto.api.v1.PayRequest
import com.senbax.senvend.proto.api.v1.PayResponse
import com.senbax.senvend.proto.api.v1.payGoodsIssued
import com.senbax.senvend.proto.api.v1.payRequest
import com.senbax.senvend.proto.api.v1.payStart
import com.senbax.senvend.proto.local.v1.PayServiceGrpcKt
import io.grpc.StatusException
import io.grpc.netty.NettyChannelBuilder
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.onStart
import kotlinx.coroutines.runBlocking
import kotlin.coroutines.cancellation.CancellationException
import kotlin.system.exitProcess

//class Main {
fun main() {
    //Reading parameters from environment variables
    val ip = System.getenv("TERMINAL_IP")
    if(ip == null) {
        println("TERMINAL_IP enviroment variable is not set!")
        exitProcess(1)
    }

    val p = System.getenv("TERMINAL_PORT") ?: "11111"
    val port: Int
    try {
        port = Integer.parseInt(p)
    } catch (_: NumberFormatException) {
        println("TERMINAL_PORT enviroment variable is not a valid integer!")
        exitProcess(1)
    }

    //Create channel to connect to device
    val channel = NettyChannelBuilder.forAddress(ip, port).usePlaintext().build()

    //Instantiate service stub with that channel
    val payClient = PayServiceGrpcKt.PayServiceCoroutineStub(channel)

    //this will be used to send requests to the server from the receiving callback
    val sender = MutableSharedFlow<PayRequest>(1)

    try {
        runBlocking {
            payClient
                //.withDeadlineAfter(2, TimeUnit.MINUTES)
                .pay(sender.asSharedFlow())
                .onStart {
                    println("Sending Request...")
                    sender.emit(payRequest {
                        start = payStart { amount = 100 }
                    })
                }
                .collect {
                    when (it.resultCase) {
                        PayResponse.ResultCase.APPROVED -> {
                            println("Approved.")
                            println("Sending goodsIssued...")
                            sender.emit(payRequest { goodsIssued = payGoodsIssued { } })
                        }

                        PayResponse.ResultCase.SUCCESS -> {
                            println("Success.")
                            cancel()
                        }

                        PayResponse.ResultCase.API_SUCCESS -> {
                            println("API_SUCCESS: $it")
                        }

                        else -> {
                            println("Error:")
                            println(it)
                            cancel()
                        }
                    }
                }
        }
    } catch(ex: StatusException) {
        //DEADLINE will happen here
        println("StatusException: $ex")
    } catch (_: CancellationException) {
        //This is reached once the rpc is finished (since we explicitly cancel it)
    }
}