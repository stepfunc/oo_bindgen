using System;
using dnp3rs;

class MainClass
{
    class TestLogger : Logger
    {
        public void on_message(LogLevel level, string message)
        {
            Console.WriteLine(level + ": " + message);
        }
    }

    class TestListener : ClientStateListener
    {
        public void on_change(ClientState state)
        {
            Console.WriteLine(state);
        }
    }

    class TestReadHandler : ReadHandler
    {
        public void begin_fragment(ResponseHeader header)
        {
            Console.WriteLine("Beginning fragment");
        }

        public void end_fragment(ResponseHeader header)
        {
            Console.WriteLine("End fragment");
        }
    }

    static void Main(string[] args)
    {
        Logging.SetLogLevel(LogLevel.Info);
        Logging.SetHandler(new TestLogger());

        using (var runtime = new Runtime(new RuntimeConfig { num_core_threads = 2 }))
        {
            var master = runtime.add_master_tcp(
                1,
                DecodeLogLevel.ObjectValues,
                new ReconnectStrategy
                {
                    min_delay = TimeSpan.FromMilliseconds(100),
                    max_delay = TimeSpan.FromSeconds(5),
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
                    disable_unsol_classes = new EventClasses
                    {
                        class1 = true,
                        class2 = true,
                        class3 = true,
                    },
                    enable_unsol_classes = new EventClasses
                    {
                        class1 = true,
                        class2 = true,
                        class3 = true,
                    },
                    auto_time_sync = AutoTimeSync.LAN,
                },
                new AssociationHandlers
                {
                    integrity_handler = readHandler,
                    unsolicited_handler = readHandler,
                    default_poll_handler = readHandler,
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
