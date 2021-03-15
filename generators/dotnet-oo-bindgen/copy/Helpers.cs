using System;
using System.Runtime.InteropServices;

namespace Helpers
{
    internal static class RustString
    {
        internal static IntPtr ToNative(string value)
        {
            var bytes = System.Text.Encoding.UTF8.GetBytes(value);
            var handle = Marshal.AllocHGlobal(bytes.Length + 1);
            // copy the bytes of the string
            Marshal.Copy(bytes, 0, handle, bytes.Length);
            // null terminator
            Marshal.WriteByte(handle, bytes.Length, 0);
            return handle;
        }

        internal static void Destroy(IntPtr value)
        {
            Marshal.FreeHGlobal(value);
        }

        internal static string FromNative(IntPtr value)
        {
            // figure out the length of the string by looking for the NULL terminator
            int length = 0;
            while (Marshal.ReadByte(value, length) != 0) ++length;
            byte[] buffer = new byte[length];
            // copy from the native type into the byte buffer
            Marshal.Copy(value, buffer, 0, length);
            return System.Text.Encoding.UTF8.GetString(buffer);
        }
    }
}