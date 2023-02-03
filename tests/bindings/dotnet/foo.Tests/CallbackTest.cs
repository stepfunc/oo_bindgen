using System;
using Xunit;
using foo;
using System.Collections.Generic;

namespace foo.Tests
{
    class CallbackImpl : ICallbackInterface
    {
        public uint lastValue = 0;
        public TimeSpan lastDuration = TimeSpan.MinValue;
        public Names name = null;
        public System.Collections.Generic.IList<Names> names = new System.Collections.Generic.List<Names>();

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

        public void OnNames(Names names)
        {
            this.name = names;
        }

        void ICallbackInterface.OnSeveralNames(ICollection<Names> names)
        {
            foreach(var name in names)
            {
                this.names.Add(name);
            }
        }
    }

    class CallbackFinalizerCounterImpl : ICallbackInterface
    {
        private readonly Counters counters;

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

        public void OnNames(Names names)
        {
            
        }

        void ICallbackInterface.OnSeveralNames(ICollection<Names> names)
        {

        }

        ~CallbackFinalizerCounterImpl()
        {
            this.counters.numFinalizersCalled++;
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
            using var cbSource = new CallbackSource();
            
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

            Assert.Null(cb.name);
            cbSource.InvokeOnNames(new Names("John", "Smith"));
            Assert.Equal("John", cb.name.FirstName);
            Assert.Equal("Smith", cb.name.LastName);

            Assert.Empty(cb.names);
            cbSource.InvokeOnSeveralNames();
            Assert.Equal(2, cb.names.Count);
            Assert.Equal("jane", cb.names[0].FirstName);
            Assert.Equal("doe", cb.names[0].LastName);
            Assert.Equal("jake", cb.names[1].FirstName);
            Assert.Equal("sully", cb.names[1].LastName);
        }

        private void SingleRun(Counters counters)
        {
            using var cbSource = new CallbackSource();
            cbSource.SetInterface(new CallbackFinalizerCounterImpl(counters));
            cbSource.SetValue(76);
        }

        [Fact]
        public void CallbackMemoryLeakTest()
        {
            var counters = new Counters();
            var numRuns = 1000;

            for (int i = 0; i < numRuns; i++)
            {
                SingleRun(counters);
            }

            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();

            Assert.Equal(numRuns, counters.numConstructorsCalled);
            Assert.Equal(numRuns, counters.numFinalizersCalled);
        }
    }
}
