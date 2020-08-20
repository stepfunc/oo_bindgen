using System;
using Xunit;
using foo;

namespace foo.Tests
{
    class CallbackImpl : CallbackInterface
    {
        public uint lastValue = 0;
        public TimeSpan lastDuration = TimeSpan.MinValue;

        public void OnValue(uint value)
        {
            lastValue = value;
        }

        public void OnDuration(TimeSpan value)
        {
            lastDuration = value;
        }
    }

    class CallbackFinalizerCounterImpl : CallbackInterface
    {
        private Counters counters;

        public void OnValue(uint value) {}
        public void OnDuration(TimeSpan value) {}

        public CallbackFinalizerCounterImpl(Counters counters)
        {
            this.counters = counters;
            this.counters.numConstructorsCalled++;
        }

        ~CallbackFinalizerCounterImpl()
        {
            this.counters.numFinalizersCalled++;
        }
    }

    class OneTimeCallbackImpl : OneTimeCallbackInterface
    {
        public uint lastValue = 0;

        public void OnValue(uint value)
        {
            lastValue = value;
        }
    }

    class Counters
    {
        public int numConstructorsCalled = 0;
        public int numFinalizersCalled = 0;
    }

    public class CallbackTests
    {
        [Fact]
        public void CallbackTest()
        {
            using (var cbSource = new CallbackSource())
            {
                var cb = new CallbackImpl();
                cbSource.AddFunc(cb);

                Assert.Equal(0u, cb.lastValue);
                cbSource.SetValue(76);
                Assert.Equal(76u, cb.lastValue);

                Assert.Equal(TimeSpan.MinValue, cb.lastDuration);
                cbSource.SetDuration(TimeSpan.FromSeconds(76));
                Assert.Equal(TimeSpan.FromSeconds(76), cb.lastDuration);

                var oneTimeCb = new OneTimeCallbackImpl();
                cbSource.AddOneTimeFunc(oneTimeCb);
                Assert.Equal(76u, oneTimeCb.lastValue);
            }
        }

        private void singleRun(Counters counters)
        {
            using (var cbSource = new CallbackSource())
            {
                cbSource.AddFunc(new CallbackFinalizerCounterImpl(counters));
                cbSource.SetValue(76);
            }
        }

        [Fact]
        public void CallbackMemoryLeakTest()
        {
            var counters = new Counters();
            var numRuns = 1000;

            for (int i = 0; i < numRuns; i++)
            {
                singleRun(counters);
            }

            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();

            Assert.Equal(numRuns, counters.numConstructorsCalled);
            Assert.Equal(numRuns, counters.numFinalizersCalled);
        }
    }
}
