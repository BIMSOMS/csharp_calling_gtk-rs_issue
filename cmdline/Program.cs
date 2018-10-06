using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.InteropServices;

namespace cmdline
{
    class Program
    {
        [DllImport("test_basic.dll")]
        private static extern Int32 add_numbers(Int32 number1, Int32 number2);
        [DllImport("test_basic.dll")]
        private static extern Int32 get_it_done();
        
        static void Main(string[] args)
        {
            //int gdn = get_it_done();
            Int32 numresult = add_numbers(2, 3);


            Console.WriteLine(numresult);
            //Console.ReadLine();
        }
    }
}
