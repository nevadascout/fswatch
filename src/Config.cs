namespace fswatch
{
    public class Config
    {
        public bool Verbose { get; set; }

        public string Path { get; set; }

        public FolderPair[] FolderPairs { get; set; }
    }
}
