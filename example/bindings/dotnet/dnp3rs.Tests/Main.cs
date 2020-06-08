using System;
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

        public void HandleBinary(HeaderInfo info, BinaryIterator it)
        {
            Console.WriteLine("Binaries:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            for (Binary? value = it.Next(); value != null; value = it.Next())
            {
                var val = (Binary)value;
                Console.WriteLine($"BI {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
                Console.WriteLine($"IsRestart: {val.Flags.IsSet(Flag.Restart)}");
            }
        }

        public void HandleDoubleBitBinary(HeaderInfo info, DoubleBitBinaryIterator it)
        {
            Console.WriteLine("Double Bit Binaries:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            for (DoubleBitBinary? value = it.Next(); value != null; value = it.Next())
            {
                var val = (DoubleBitBinary)value;
                Console.WriteLine($"DBBI {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleBinaryOutputStatus(HeaderInfo info, BinaryOutputStatusIterator it)
        {
            Console.WriteLine("Binary Output Statuses:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            for (BinaryOutputStatus? value = it.Next(); value != null; value = it.Next())
            {
                var val = (BinaryOutputStatus)value;
                Console.WriteLine($"BOS {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleCounter(HeaderInfo info, CounterIterator it)
        {
            Console.WriteLine("Counters:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            for (Counter? value = it.Next(); value != null; value = it.Next())
            {
                var val = (Counter)value;
                Console.WriteLine($"Counter {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleFrozenCounter(HeaderInfo info, FrozenCounterIterator it)
        {
            Console.WriteLine("Frozen Counters:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            for (FrozenCounter? value = it.Next(); value != null; value = it.Next())
            {
                var val = (FrozenCounter)value;
                Console.WriteLine($"Frozen Counter {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleAnalog(HeaderInfo info, AnalogIterator it)
        {
            Console.WriteLine("Analogs:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            for (Analog? value = it.Next(); value != null; value = it.Next())
            {
                var val = (Analog)value;
                Console.WriteLine($"AI {val.Index}: Value={val.Value} Flags={val.Flags.Value} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }

        public void HandleAnalogOutputStatus(HeaderInfo info, AnalogOutputStatusIterator it)
        {
            Console.WriteLine("Analog Output Statuses:");
            Console.WriteLine("Qualifier: " + info.Qualifier);
            Console.WriteLine("Variation: " + info.Variation);

            for (AnalogOutputStatus? value = it.Next(); value != null; value = it.Next())
            {
                var val = (AnalogOutputStatus)value;
                Console.WriteLine($"AOS {val.Index}: Value={val.Value} Flags={val.Flags} Time={val.Time.Value} ({val.Time.Quality})");
            }
        }
    }

    static void Main(string[] args)
    {
        Logging.SetLogLevel(LogLevel.Info);
        Logging.SetHandler(new TestLogger());

        using (var runtime = new Runtime(new RuntimeConfig { NumCoreThreads = 2 }))
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
                switch (Console.ReadLine())
                {
                    case "x":
                        return;
                    default:
                        break;
                }
            }
        }
    }
}
