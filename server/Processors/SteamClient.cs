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

    [Processor("steamid", Verb.Get)]
    public object GetSteamId()
    {
        return new {
            SteamId64 = Steamworks.SteamClient.SteamId.Value,
            Steamworks.SteamClient.SteamId.AccountId,
            Steamworks.SteamClient.SteamId.IsValid,
        };
    }

    [Processor("level", Verb.Get)]
    public int GetSteamLevel()
    {
        return Steamworks.SteamUser.SteamLevel;
    }
    
    [Processor("ipcountry", Verb.Get)]
    public string GetIpCountry()
    {
        return Steamworks.SteamUtils.IpCountry;
    }
}