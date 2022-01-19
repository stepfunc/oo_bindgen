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

    internal static class PrimitivePointer
   {
       internal static bool ReadBool(IntPtr x)
       {
           return Unsigned.ReadByte(x) != 0;
       }

       internal static float ReadFloat(IntPtr x)
       {
           if (x == IntPtr.Zero)
           {
               throw new ArgumentException("IntPtr cannot be zero");
           }
           var bytes = new byte[4];
           Marshal.Copy(x, bytes, 0, 4);
           return BitConverter.ToSingle(bytes, 0);
       }

       internal static double ReadDouble(IntPtr x)
       {
           if (x == IntPtr.Zero)
           {
               throw new ArgumentException("IntPtr cannot be zero");
           }
           var bytes = new byte[8];
           Marshal.Copy(x, bytes, 0, 8);
           return BitConverter.ToDouble(bytes, 0);
       }

       internal static class Signed
       {
           internal static sbyte ReadByte(IntPtr x)
           {
               return unchecked((sbyte)Unsigned.ReadByte(x));
           }

           internal static short ReadShort(IntPtr x)
           {
               if (x == IntPtr.Zero)
               {
                   throw new ArgumentException("IntPtr cannot be zero");
               }
               return Marshal.ReadInt16(x);
           }

           internal static int ReadInt(IntPtr x)
           {
               if (x == IntPtr.Zero)
               {
                   throw new ArgumentException("IntPtr cannot be zero");
               }
               return Marshal.ReadInt32(x);
           }

           internal static long ReadLong(IntPtr x)
           {
               if (x == IntPtr.Zero)
               {
                   throw new ArgumentException("IntPtr cannot be zero");
               }
               return Marshal.ReadInt64(x);
           }
       }

       internal static class Unsigned
       {
           internal static byte ReadByte(IntPtr x)
           {
               if(x == IntPtr.Zero)
               {
                   throw new ArgumentException("IntPtr cannot be zero");
               }
               return Marshal.ReadByte(x);
           }

           internal static ushort ReadShort(IntPtr x)
           {
               return unchecked((ushort)Signed.ReadShort(x));
           }

           internal static uint ReadInt(IntPtr x)
           {
               return unchecked((uint)Signed.ReadInt(x));
           }

           internal static ulong ReadLong(IntPtr x)
           {
               return unchecked((ulong)Signed.ReadLong(x));
           }
       }
   }
}