using System;
using Xunit;
using foo;
using System.Threading;

namespace foo.Tests
{
    class CallbackImpl : CallbackInterface
    {
        public uint lastValue = 0;

        public void on_value(uint value)
        {
            lastValue = value;
        }
    }

    class CallbackFinalizerCounterImpl : CallbackInterface
    {
        public static int numConstructorsCalled = 0;
        public static int numFinalizersCalled = 0;

        public void on_value(uint value) {}

        public CallbackFinalizerCounterImpl()
        {
            CallbackFinalizerCounterImpl.numConstructorsCalled++;
        }

        ~CallbackFinalizerCounterImpl()
        {
            CallbackFinalizerCounterImpl.numFinalizersCalled++;
        }
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
            }
        }

        [Fact]
        public void CallbackMemoryLeakTest()
        {
            var numRuns = 1000;

            for (int i = 0; i < numRuns; i++)
            {
                using (var cbSource = new CallbackSource())
                {
                    cbSource.AddFunc(new CallbackFinalizerCounterImpl());
                    cbSource.SetValue(76);
                }
            }

            GC.Collect();
            GC.WaitForPendingFinalizers();

            Assert.Equal(numRuns, CallbackFinalizerCounterImpl.numConstructorsCalled);
            Assert.Equal(numRuns, CallbackFinalizerCounterImpl.numFinalizersCalled);
        }
    }
}
