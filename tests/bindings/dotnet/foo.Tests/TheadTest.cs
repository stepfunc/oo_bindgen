using System;
using Xunit;
using foo;
using System.Collections.Generic;

namespace foo.Tests
{
    class Listener : IValueChangeListener
    {
        public List<UInt32> values = new List<uint>();

        void IValueChangeListener.OnValueChange(uint value)
        {
            values.Add(value);
        }
    }


    public class ThreadTest
    {   
        [Fact]
        public async void AsyncCallbacksWork()
        {
           var listener = new Listener();
           var tc = new foo.ThreadClass(42, listener);
           var result = await tc.Add(4);
           Assert.Equal(46u, result);
           tc.Update(43);

           // shutdown the thread explicitly instead of waiting for GC
           tc.Shutdown();
            // this allows us to safely check the listener values
            Assert.Equal(2, listener.values.Count);
            Assert.Equal(46u, listener.values[0]);
            Assert.Equal(43u, listener.values[1]);
        }       
    }
}
