namespace fswatch
{
    using System;
    using System.Collections.Generic;
    using System.Linq;

    internal class Program
    {
        private static void Main(string[] args)
        {
            if (!args.Any() || args.Length > 1)
            {
                Console.WriteLine("Error: Missing required parameter!");
                Console.WriteLine("Usage: fswatch <path-to-config-file>");
                return;
            }

            var configLoader = new ConfigLoader(args[0]);
            if (!configLoader.ConfigFileExists())
            {
                Console.WriteLine("Error: The specified config file does not exist!");
                return;
            }

            var config = configLoader.Load();
            if (config == null) return;

            var log = new Logger(config.Verbose);
            log.Write($"Loaded config file {config.Path}");

            var watchers = new List<FsWatcher>();

            foreach (var folderPair in config.FolderPairs)
            {
                var watcher = new FsWatcher(folderPair);
                watcher.Watch(log);

                // May (not?) be needed to persist the watcher during garbage collection
                watchers.Add(watcher);
            }

            // DEBUG
            Console.WriteLine("Press q to exit");
            while (Console.Read() != 'q') ;
        }

        private static int Run(string[] args)
        {
            return 0;
        }
    }
}
