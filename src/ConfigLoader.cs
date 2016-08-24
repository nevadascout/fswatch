namespace fswatch
{
    using System;
    using System.IO;

    using Newtonsoft.Json;

    public class ConfigLoader
    {
        public string Path { get; set; }

        public ConfigLoader(string path)
        {
            this.Path = path;
        }

        public bool ConfigFileExists()
        {
            return File.Exists(this.Path);
        }

        public Config Load()
        {
            var fileText = File.ReadAllText(this.Path);
            Config config = null;

            try
            {
                config = JsonConvert.DeserializeObject<Config>(fileText);
                config.Path = this.Path;
            }
            catch (JsonException e)
            {
                Console.WriteLine("Error parsing config file!");
                Console.WriteLine(e.Message);
            }

            return config;
        }
    }
}
