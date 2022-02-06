namespace Apigelo.Models;

public record AnimeEpisode(int Number, string Title, string Path, string Description, string Thumbnail);
public record AnimeSeason(int Number, string Path, string Poster, IEnumerable<AnimeEpisode> Episodes);
public record Anime(int Id, string Title, string Path, string Backdrop, string Poster, string Description);
public record AnimeCollection(IEnumerable<Anime> Collection);