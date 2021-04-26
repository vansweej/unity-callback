using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;
using System.Runtime.InteropServices;

public class nativePlugin : MonoBehaviour
{
    [DllImport ("nativeplugin")]
    private static extern void init ();
    [DllImport ("nativeplugin")]
    private static extern int pow (int n);
    [DllImport ("nativeplugin")]
    public static extern int create(IntPtr readFn);
    [DllImport ("nativeplugin")]
    public static extern int trigger(int x);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void MyDelegate(IntPtr str);

    public static void CallBackFunction(IntPtr str) {

        Debug.Log("::CallBaaaaaaack : " + Marshal.PtrToStringAnsi(str)); 
    }

    public static MyDelegate callback_delegate = CallBackFunction;

    // Start is called before the first frame update
    void Start()
    {
        var ptr = Marshal.GetFunctionPointerForDelegate(callback_delegate);
    
        create(ptr);
//        trigger(42);

        
    }

    // Update is called once per frame
    void Update()
    {
        Debug.Log("pow will be called");
        int x = pow(5);

    
    }
}
