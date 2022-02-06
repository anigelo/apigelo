using Apigelo.Models;
using Newtonsoft.Json;

namespace Apigelo.Services;

public class FileService
{
    private readonly ILogger<FileService> logger;

    public FileService(ILogger<FileService> logger)
    {
        this.logger = logger;
    }
    public IEnumerable<Anime> GetAnimes()
    {
        var dataPath = Environment.GetEnvironmentVariable("APIGELO_DATA_PATH");
        if (dataPath == null)
        {
            return Array.Empty<Anime>();
        }
        
        var indexPath = Path.Combine(dataPath, "index.json");
        logger.LogInformation("Metadata index: {}", indexPath);
        var fileContent = File.ReadAllText(indexPath);
        var collection = JsonConvert.DeserializeObject<AnimeCollection>(fileContent);

        return collection?.Collection ?? Array.Empty<Anime>();
    }
}