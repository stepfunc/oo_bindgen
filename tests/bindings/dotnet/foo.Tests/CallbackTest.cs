using System;
using Xunit;
using foo;

namespace foo.Tests
{
    class CallbackImpl : ICallbackInterface
    {
        public uint lastValue = 0;
        public TimeSpan lastDuration = TimeSpan.MinValue;

        public uint OnValue(uint value)
        {
            lastValue = value;
            return value;
        }

        public TimeSpan OnDuration(TimeSpan value)
        {
            lastDuration = value;
            return value;
        }
    }

    class CallbackFinalizerCounterImpl : ICallbackInterface
    {
        private Counters counters;

        public uint OnValue(uint value)
        {
            return value;
        }

        public TimeSpan OnDuration(TimeSpan value)
        {
            return value;
        }

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

    class OneTimeCallbackImpl : IOneTimeCallbackInterface
    {
        public uint lastValue = 0;

        public uint OnValue(uint value)
        {
            lastValue = value;
            return value;
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
                cbSource.SetInterface(cb);

                Assert.Equal(0u, cb.lastValue);
                var result = cbSource.SetValue(76);
                Assert.Equal(76u, result);
                Assert.Equal(76u, cb.lastValue);

                Assert.Equal(TimeSpan.MinValue, cb.lastDuration);
                var timeResult = cbSource.SetDuration(TimeSpan.FromSeconds(76));
                Assert.Equal(TimeSpan.FromSeconds(76), timeResult);
                Assert.Equal(TimeSpan.FromSeconds(76), cb.lastDuration);

                var oneTimeCb = new OneTimeCallbackImpl();
                result = cbSource.CallOneTime(oneTimeCb);
                Assert.Equal(76u, result);
                Assert.Equal(76u, oneTimeCb.lastValue);
            }
        }

        private void singleRun(Counters counters)
        {
            using (var cbSource = new CallbackSource())
            {
                cbSource.SetInterface(new CallbackFinalizerCounterImpl(counters));
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
