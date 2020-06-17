using System;
using System.Threading.Tasks;
using System.Collections.Immutable;
using dnp3rs;

class MainClass
{
    class TestLogger : Logger
    {
        public void OnMessage(LogLevel level, string message)
        {
            //Console.WriteLine($"{level}: {message}");
        }
    }

    class TestListener : ClientStateListener
    {
        public void OnChange(ClientState state)
        {
            Console.WriteLine(state);
        }
    }

    class TestReadHandler : ReadHandler
    {
        public void BeginFragment(ResponseHeader header)
        {
            Console.WriteLine("Beginning fragment");
            Console.WriteLine($"Is broadcast: {header.Iin.Iin1.IsSet(Iin1Flag.Broadcast)}");
        }

        public void EndFragment(ResponseHeader header)
        {
            Console.WriteLine("End fragment");
        }

        public void HandleBinary(HeaderInfo info, ImmutableArray<Binary> values)
        {
            Console.WriteLine("Binaries:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            foreach (var val in values)
            {
                Console.WriteLine($"BI {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
                Console.WriteLine($"IsRestart: {val.Flags.IsSet(Flag.Restart)}");
            }
        }

        public void HandleDoubleBitBinary(HeaderInfo info, ImmutableArray<DoubleBitBinary> values)
        {
            Console.WriteLine("Double Bit Binaries:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            foreach (var val in values)
            {
                Console.WriteLine($"DBBI {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleBinaryOutputStatus(HeaderInfo info, ImmutableArray<BinaryOutputStatus> values)
        {
            Console.WriteLine("Binary Output Statuses:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            foreach (var val in values)
            {
                Console.WriteLine($"BOS {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleCounter(HeaderInfo info, ImmutableArray<Counter> values)
        {
            Console.WriteLine("Counters:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            foreach (var val in values)
            {
                Console.WriteLine($"Counter {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleFrozenCounter(HeaderInfo info, ImmutableArray<FrozenCounter> values)
        {
            Console.WriteLine("Frozen Counters:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            foreach (var val in values)
            {
                Console.WriteLine($"Frozen Counter {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleAnalog(HeaderInfo info, ImmutableArray<Analog> values)
        {
            Console.WriteLine("Analogs:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            foreach (var val in values)
            {
                Console.WriteLine($"AI {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleAnalogOutputStatus(HeaderInfo info, ImmutableArray<AnalogOutputStatus> values)
        {
            Console.WriteLine("Analog Output Statuses:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            foreach (var val in values)
            {
                Console.WriteLine($"AOS {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }
    }

    public static void Main(string[] args)
    {   
            MainAsync().GetAwaiter().GetResult();
    }

    private static async Task MainAsync()
    {
        Logging.SetLogLevel(LogLevel.Info);
        Logging.SetHandler(new TestLogger());

        using (var runtime = new Runtime(new RuntimeConfig { NumCoreThreads = 4 }))
        {
            var master = runtime.AddMasterTcp(
                1,
                DecodeLogLevel.ObjectValues,
                new ReconnectStrategy
                {
                    MinDelay = TimeSpan.FromMilliseconds(100),
                    MaxDelay = TimeSpan.FromSeconds(5),
                },
                TimeSpan.FromSeconds(5),
                "127.0.0.1:20000",
                new TestListener()
            );

            var readHandler = new TestReadHandler();
            var association = master.AddAssociation(
                1024,
                new AssociationConfiguration
                {
                    DisableUnsolClasses = new EventClasses
                    {
                        Class1 = true,
                        Class2 = true,
                        Class3 = true,
                    },
                    EnableUnsolClasses = new EventClasses
                    {
                        Class1 = true,
                        Class2 = true,
                        Class3 = true,
                    },
                    AutoTimeSync = AutoTimeSync.Lan,
                },
                new AssociationHandlers
                {
                    IntegrityHandler = readHandler,
                    UnsolicitedHandler = readHandler,
                    DefaultPollHandler = readHandler,
                }
            );

            while (true)
            {
                switch (await GetInputAsync())
                {
                    case "x":
                        return;
                    case "lts":
                        {
                            var result = await association.PerformTimeSync(TimeSyncMode.Lan);
                            if (result != TimeSyncResult.Success)
                            {
                                Console.WriteLine($"Error: {result}");
                            }
                            break;
                        }
                    case "nts":
                        {
                            var result = await association.PerformTimeSync(TimeSyncMode.NonLan);
                            if (result != TimeSyncResult.Success)
                            {
                                Console.WriteLine($"Error: {result}");
                            }
                            break;
                        }
                    default:
                        break;
                }
            }
        }
    }

    private static Task<string> GetInputAsync()
    {
        return Task.Run(() => Console.ReadLine());
    }
}
