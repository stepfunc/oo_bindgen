using System;
using System.Runtime.InteropServices;

namespace Helpers
{
    internal class RustString
    {
        internal static IntPtr Allocate(string value)
        {
            var bytes = System.Text.Encoding.UTF8.GetBytes(value);
            var handle = Marshal.AllocHGlobal(bytes.Length + 1);
            // copy the bytes of the string
            Marshal.Copy(bytes, 0, handle, bytes.Length);
            // null terminator
            Marshal.WriteByte(handle, bytes.Length, 0);
            return handle;
        }
    }
}