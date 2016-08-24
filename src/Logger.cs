namespace fswatch
{
    using System;

    public class Logger
    {
        public bool Verbose { get; set; }

        public Logger(bool verbose)
        {
            this.Verbose = verbose;
        }

        public void Write(string message)
        {
            if (this.Verbose)
            {
                Console.WriteLine(message);
            }
        }
    }
}
