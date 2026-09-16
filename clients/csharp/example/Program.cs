using com.senbax.senvend.proto.Api.V1;
using com.senbax.senvend.proto.Local.V1;
using Grpc.Core;
using Grpc.Net.Client;

// Reading parameters from environment variables
var ip = Environment.GetEnvironmentVariable("TERMINAL_IP");
if (string.IsNullOrWhiteSpace(ip))
{
    Console.Error.WriteLine("TERMINAL_IP environment variable is not set!");
    return 1;
}

var portVariable = Environment.GetEnvironmentVariable("TERMINAL_PORT") ?? "11111";
if (!int.TryParse(portVariable, out var port))
{
    Console.Error.WriteLine("TERMINAL_PORT environment variable is not a valid integer!");
    return 1;
}

// Create channel to connect to device.
using var channel = GrpcChannel.ForAddress($"http://{ip}:{port}");

// Instantiate service stub with that channel
var payClient = new PayService.PayServiceClient(channel);

try
{
    // .Pay(deadline: DateTime.UtcNow.AddMinutes(2))
    using var call = payClient.Pay();

    Console.WriteLine("Sending Request...");
    await call.RequestStream.WriteAsync(new PayRequest { Start = new PayStart { Amount = 100 } });

    await foreach (var response in call.ResponseStream.ReadAllAsync())
    {
        switch (response.ResultCase)
        {
            case PayResponse.ResultOneofCase.Approved:
                Console.WriteLine("Approved.");
                Console.WriteLine("Sending goodsIssued...");
                await call.RequestStream.WriteAsync(new PayRequest { GoodsIssued = new PayGoodsIssued() });
                break;

            case PayResponse.ResultOneofCase.Success:
                Console.WriteLine("Success.");
                await call.RequestStream.CompleteAsync();
                return 0;

            case PayResponse.ResultOneofCase.ApiSuccess:
                Console.WriteLine($"API_SUCCESS: {response}");
                break;

            default:
                Console.WriteLine("Error:");
                Console.WriteLine(response);
                await call.RequestStream.CompleteAsync();
                return 1;
        }
    }
}
catch (RpcException ex)
{
    // DEADLINE will happen here
    Console.Error.WriteLine($"RpcException: {ex}");
    return 1;
}

return 2;
