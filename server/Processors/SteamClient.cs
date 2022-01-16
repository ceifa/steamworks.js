using Airmiss.Processor;

[ProcessorHub("client")]
public class SteamClient
{
    [Processor("init/{appId}", Verb.Post)]
    public void Initialize([Path("appId")] uint appId)
    {
        Steamworks.SteamClient.Init(appId);
    }

    [Processor("name", Verb.Get)]
    public string GetName()
    {
        return Steamworks.SteamClient.Name;
    }
}