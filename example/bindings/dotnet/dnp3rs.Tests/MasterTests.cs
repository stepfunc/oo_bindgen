using System;
using Xunit;
using dnp3rs;

namespace dnp3rs.Tests
{
    public class MasterTests
    {
        class TestListener : ClientStateListener
        {
            public void on_change(ClientState state)
            {
                Console.WriteLine(state);
            }
        }

        [Fact]
        public void DurationZeroTest()
        {
            var config = new RuntimeConfig();
            config.num_core_threads = 2;

            using(var runtime = new Runtime(config))
            {
                var strategy = new ReconnectStrategy();
                strategy.min_delay = TimeSpan.FromMilliseconds(100);
                strategy.max_delay = TimeSpan.FromSeconds(5);
                var master = runtime.add_master_tcp(1024, DecodeLogLevel.ObjectValues, strategy, TimeSpan.FromSeconds(5), "127.0.0.1:8000", new TestListener());
            }
        }
    }
}
