using System;
using Xunit;
using foo;
using System.Collections.Generic;

namespace foo.Tests
{  
    public class ThreadTest
    {   
        [Fact]
        public async void AsyncCallbacksWork()
        {
            var values = new List<uint>();
            var tc = new foo.ThreadClass(42, new ValueChangeListener(item => values.Add(item)));
            var result = await tc.Add(4);
            Assert.Equal(46u, result);
            tc.Update(43);
            tc.Execute(new Operation(x => 2 * x));

            // shutdown the thread explicitly instead of waiting for GC
            tc.Shutdown();
            // this allows us to safely check the listener values
            Assert.Equal(3, values.Count);
            Assert.Equal(46u, values[0]);
            Assert.Equal(43u, values[1]);
            Assert.Equal(86u, values[2]);
        }       
    }
}
