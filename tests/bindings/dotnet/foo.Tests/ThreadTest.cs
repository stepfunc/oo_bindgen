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
            var tc = new foo.ThreadClass(42, item => values.Add(item));
            var result = await tc.Add(4);
            Assert.Equal(46u, result);
            tc.Update(43);
            tc.Execute(x => 2 * x);

            // shutdown the thread explicitly instead of waiting for GC
            tc.Shutdown();
            // this allows us to safely check the listener values
            Assert.Equal(3, values.Count);
            Assert.Equal(46u, values[0]);
            Assert.Equal(43u, values[1]);
            Assert.Equal(86u, values[2]);
        }

        [Fact]
        public async void TaskFailuresWorkAsExpected()
        {
            var values = new List<uint>();
            var tc = new foo.ThreadClass(42, item => values.Add(item));
            tc.QueueError(MathIsBroken.MathIsBroke);

            try
            {
                var result = await tc.Add(43);
                Assert.True(false);
            }
            catch (BrokenMathException ex)
            {
                Assert.Equal(MathIsBroken.MathIsBroke, ex.error);
            }
            finally
            {
                tc.Shutdown();
            }

            Assert.Empty(values);
        }
    }
}
