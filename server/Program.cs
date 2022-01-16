using Airmiss;
using Airmiss.Protocol.Tcp;

var runner = new AirmissConfiguration()
    .Protocol.Tcp("127.0.0.1", 45162)
    .Processor.AddCurrentAssembly()
    .GetRunner();

await runner.StartAsync();

Console.WriteLine("Steamworks.js server started.");
Console.ReadKey();