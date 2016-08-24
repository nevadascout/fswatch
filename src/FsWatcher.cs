namespace fswatch
{
    using System.IO;

    public class FsWatcher
    {
        public FolderPair FolderPair { get; set; }

        public FileSystemWatcher FileSystemWatcher { get; set; }

        public Logger Log { get; set; }

        public FsWatcher(FolderPair folderPair)
        {
            this.FolderPair = folderPair;
        }

        public void Watch(Logger log)
        {
            this.Log = log;
            this.Log.Write($"Starting watcher for folder {this.FolderPair.Source}");
            this.FileSystemWatcher = new FileSystemWatcher
                                         {
                                             Path = this.FolderPair.Source,
                                             IncludeSubdirectories = true,
                                             NotifyFilter = NotifyFilters.Attributes |
                                                            NotifyFilters.CreationTime |
                                                            NotifyFilters.DirectoryName |
                                                            NotifyFilters.FileName |
                                                            NotifyFilters.LastAccess |
                                                            NotifyFilters.LastWrite |
                                                            NotifyFilters.Security |
                                                            NotifyFilters.Size,
                                             Filter = "*.*",
                                             EnableRaisingEvents = true
                                         };

            // Hook into FSW events
            this.FileSystemWatcher.Changed += this.OnChanged;
            this.FileSystemWatcher.Created += this.OnChanged;
            this.FileSystemWatcher.Deleted += this.OnChanged;
            this.FileSystemWatcher.Renamed += this.OnRenamed;
            this.FileSystemWatcher.Error += this.OnError;
        }

        private void OnError(object sender, ErrorEventArgs e)
        {
            this.Log.Write($"ERROR: {e.GetException().Message}");
        }

        private void OnChanged(object source, FileSystemEventArgs e)
        {
            this.Log.Write($"{e.ChangeType}: {e.Name} ({e.FullPath})");
        }

        private void OnRenamed(object source, RenamedEventArgs e)
        {
            this.Log.Write($"Renamed: {e.OldFullPath} to {e.FullPath}");
        }
    }
}
