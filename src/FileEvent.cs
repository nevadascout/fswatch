namespace fswatch
{
    using System.IO;

    public class FileEvent
    {
        public WatcherChangeTypes ChangeType { get; set; }

        public string FilePath { get; set; }

        public string NewPath { get; set; }
    }
}
