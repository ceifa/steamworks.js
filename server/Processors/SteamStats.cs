using Airmiss.Processor;

[ProcessorHub("stats")]
public class SteamStats
{
    [Processor("{name}/float", Verb.Get)]
    public void GetStatFloat([Path("name")] string name)
    {
        Steamworks.SteamUserStats.GetStatFloat(name);
    }

    [Processor("{name}/int", Verb.Get)]
    public void GetStatInt([Path("name")] string name)
    {
        Steamworks.SteamUserStats.GetStatInt(name);
    }
    
    [Processor("{name}/float", Verb.Post)]
    public void SetStatFloat([Path("name")] string name, [Content] float value)
    {
        Steamworks.SteamUserStats.SetStat(name, value);
    }

    [Processor("{name}/int", Verb.Post)]
    public void SetStatInt([Path("name")] string name, [Content] int value)
    {
        Steamworks.SteamUserStats.SetStat(name, value);
    }
}