using Airmiss.Processor;
using System.Text;

[ProcessorHub("remotestorage")]
public class SteamRemoteStorage
{
    [Processor("enabled", Verb.Get)]
    public bool IsCloudEnabled()
    {
        return Steamworks.SteamRemoteStorage.IsCloudEnabled;
    }

    [Processor("{filename}", Verb.Delete)]
    public bool DeleteFile([Path("filename")] string filename)
    {
        return Steamworks.SteamRemoteStorage.FileDelete(filename);
    }
    
    [Processor("{filename}/exists", Verb.Get)]
    public bool HasFile([Path("filename")] string filename)
    {
        return Steamworks.SteamRemoteStorage.FileExists(filename);
    }

    [Processor("{filename}", Verb.Get)]
    public string ReadFile([Path("filename")] string filename)
    {
        return Encoding.UTF8.GetString(Steamworks.SteamRemoteStorage.FileRead(filename));
    }

    [Processor("{filename}", Verb.Post)]
    public bool WriteFile([Path("filename")] string filename, [Content] string content)
    {
        return Steamworks.SteamRemoteStorage.FileWrite(filename, Encoding.UTF8.GetBytes(content));
    }
}